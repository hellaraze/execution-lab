use anyhow::{bail, Context};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Replay,
    Live,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LiveMode {
    DryRun,
    PostOnly,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Paths {
    #[serde(default = "default_state_dir")]
    pub state_dir: PathBuf,
    #[serde(default = "default_runs_dir")]
    pub runs_dir: PathBuf,
}
fn default_state_dir() -> PathBuf {
    PathBuf::from("state")
}
fn default_runs_dir() -> PathBuf {
    PathBuf::from("runs")
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Risk {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_max_notional")]
    pub max_notional: f64,
}
fn default_true() -> bool {
    true
}
fn default_max_notional() -> f64 {
    10.0
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct KillSwitch {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_kill_path")]
    pub path: PathBuf,
}
fn default_kill_path() -> PathBuf {
    PathBuf::from("state/KILL")
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProductConfigV1 {
    pub config_version: u32,
    pub mode: Mode,

    // replay
    pub eventlog: Option<PathBuf>,

    // live
    pub live_mode: Option<LiveMode>,

    #[serde(default)]
    pub paths: Paths,
    #[serde(default)]
    pub risk: Risk,
    #[serde(default)]
    pub kill_switch: KillSwitch,
}

impl ProductConfigV1 {
    pub fn load(path: &Path) -> anyhow::Result<(Self, Vec<u8>)> {
        let raw = fs::read(path).with_context(|| format!("read config: {}", path.display()))?;
        let s = std::str::from_utf8(&raw).context("config must be UTF-8")?;
        let cfg: ProductConfigV1 =
            toml::from_str(s).with_context(|| format!("parse toml: {}", path.display()))?;
        Ok((cfg, raw))
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.config_version != 1 {
            bail!(
                "unsupported config_version={} (expected 1)",
                self.config_version
            );
        }
        match self.mode {
            Mode::Replay => {
                let p = self
                    .eventlog
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("replay requires eventlog=..."))?;
                if !p.exists() {
                    bail!("eventlog does not exist: {}", p.display());
                }
                if self.live_mode.is_some() {
                    bail!("live_mode must be omitted in replay mode");
                }
            }
            Mode::Live => {
                let _lm = self
                    .live_mode
                    .ok_or_else(|| anyhow::anyhow!("live requires live_mode=dry_run|post_only"))?;
                if !self.risk.enabled {
                    bail!("live requires risk.enabled=true (cannot bypass risk)");
                }
                if self.risk.max_notional <= 0.0 {
                    bail!("risk.max_notional must be > 0");
                }
                if !self.kill_switch.enabled {
                    bail!("live requires kill_switch.enabled=true");
                }
                if self.kill_switch.path.as_os_str().is_empty() {
                    bail!("kill_switch.path must be non-empty");
                }
            }
        }
        Ok(())
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn git_head() -> String {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output();
    match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => "UNKNOWN".to_string(),
    }
}

#[derive(Debug, Serialize)]
struct RunManifest {
    ts_utc: String,
    run_id: String,
    git_head: String,
    config_sha256: String,
    mode: String,
    live_mode: Option<String>,
    eventlog: Option<String>,
}

pub fn validate_config(path: &Path) -> anyhow::Result<()> {
    let (cfg, _) = ProductConfigV1::load(path)?;
    cfg.validate()?;
    Ok(())
}

pub fn init_config(out: &Path, kind: &str) -> anyhow::Result<()> {
    let src = match kind {
        "replay" => Path::new("configs/replay.toml"),
        "live_dryrun" => Path::new("configs/live_dryrun.toml"),
        "live_postonly" => Path::new("configs/live_postonly.toml"),
        other => bail!("unknown kind={other} (expected replay|live_dryrun|live_postonly)"),
    };
    let b = fs::read(src).with_context(|| format!("read template: {}", src.display()))?;
    if let Some(p) = out.parent() {
        if !p.as_os_str().is_empty() {
            fs::create_dir_all(p)?;
        }
    }
    fs::write(out, b)?;
    Ok(())
}

pub fn run_from_config_path(config_path: &Path) -> anyhow::Result<(String, PathBuf)> {
    let (cfg, raw) = ProductConfigV1::load(config_path)?;
    cfg.validate()?;

    fs::create_dir_all(&cfg.paths.state_dir)?;
    fs::create_dir_all(&cfg.paths.runs_dir)?;

    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop2 = stop.clone();
        let _ = ctrlc::set_handler(move || {
            stop2.store(true, Ordering::SeqCst);
        });
    }

    let ts = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let run_id = format!("{}_pid{}", ts.replace(['-', ':'], ""), std::process::id());

    let run_dir = cfg.paths.runs_dir.join(&run_id);
    fs::create_dir_all(&run_dir)?;

    fs::write(run_dir.join("config.toml"), &raw)?;
    let mut decisions = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(run_dir.join("decisions.log"))?;
    writeln!(
        decisions,
        "{{\"ts\":\"{}\",\"event\":\"run_start\",\"run_id\":\"{}\"}}",
        ts, run_id
    )?;
    decisions.flush()?;

    let config_sha = sha256_hex(&raw);

    let (mode_s, live_s) = match cfg.mode {
        Mode::Replay => ("replay".to_string(), None),
        Mode::Live => (
            "live".to_string(),
            cfg.live_mode.map(|m| match m {
                LiveMode::DryRun => "dry_run".to_string(),
                LiveMode::PostOnly => "post_only".to_string(),
            }),
        ),
    };

    if let Mode::Replay = cfg.mode {
        let p = cfg.eventlog.as_ref().unwrap();
        let b = fs::read(p).with_context(|| format!("read eventlog: {}", p.display()))?;
        let h = sha256_hex(&b);
        fs::write(run_dir.join("replay_input_sha256.txt"), format!("{}\n", h))?;
        writeln!(decisions, "{{\"ts\":\"{}\",\"event\":\"replay_ok\"}}", ts)?;
        decisions.flush()?;
    } else {
        // minimal live loop: stop on ctrl+c OR kill file
        let kill = cfg.kill_switch.path.clone();
        writeln!(
            decisions,
            "{{\"ts\":\"{}\",\"event\":\"live_start\",\"live_mode\":\"{}\"}}",
            ts,
            live_s.clone().unwrap_or_else(|| "unknown".to_string())
        )?;
        decisions.flush()?;
        while !stop.load(Ordering::SeqCst) && !kill.exists() {
            thread::sleep(Duration::from_millis(200));
        }
        if kill.exists() {
            writeln!(
                decisions,
                "{{\"ts\":\"{}\",\"event\":\"kill_switch\",\"path\":\"{}\"}}",
                Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                kill.display()
            )?;
        } else {
            writeln!(
                decisions,
                "{{\"ts\":\"{}\",\"event\":\"signal_stop\"}}",
                Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            )?;
        }
        decisions.flush()?;
    }

    let manifest = RunManifest {
        ts_utc: ts,
        run_id: run_id.clone(),
        git_head: git_head(),
        config_sha256: config_sha,
        mode: mode_s,
        live_mode: live_s,
        eventlog: cfg.eventlog.as_ref().map(|p| p.display().to_string()),
    };
    fs::write(
        run_dir.join("run_manifest.json"),
        serde_json::to_vec_pretty(&manifest)?,
    )?;

    // stable sha256 list (minimal)
    let mut lines = Vec::new();
    for name in [
        "config.toml",
        "decisions.log",
        "run_manifest.json",
        "replay_input_sha256.txt",
    ] {
        let p = run_dir.join(name);
        if p.exists() {
            let b = fs::read(&p)?;
            lines.push(format!("{}  {}", sha256_hex(&b), name));
        }
    }
    fs::write(run_dir.join("sha256.txt"), lines.join("\n") + "\n")?;
    fs::write(run_dir.join("RUN_OK"), b"OK\n")?;

    Ok((run_id, run_dir))
}
