use clap::{Parser, Subcommand};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    name = "execution-lab",
    version,
    about = "Institutional-grade execution infrastructure (product CLI)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Connect an exchange (stub)
    Connect { exchange: String },

    /// Load an eventlog (stub)
    Load { eventlog: PathBuf },

    /// Run scan (stub)
    Run {
        #[arg(long, default_value = "replay")]
        mode: String,
        #[arg(long)]
        eventlog: Option<PathBuf>,
    },

    /// Show last decisions (stub)
    Show,

    /// Demo: deterministic replay on built-in fixture
    Demo,

    /// Bundle: run demo + produce evidence bundle (NO python)
    Bundle,
}

#[derive(Serialize)]
struct FileEntry {
    sha256: String,
    bytes: u64,
}

#[derive(Serialize)]
struct Manifest {
    ts_utc: String,
    app: String,
    cmd: Vec<String>,
    cwd: String,
    files: std::collections::BTreeMap<String, FileEntry>,
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn write_file(path: &Path, bytes: &[u8]) -> anyhow::Result<FileEntry> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)?;
    let meta = fs::metadata(path)?;
    Ok(FileEntry {
        sha256: sha256_hex(bytes),
        bytes: meta.len(),
    })
}

fn run_demo_to_bytes() -> anyhow::Result<Vec<u8>> {
    let eventlog = PathBuf::from("replay/tests/data/binance_depth_fixture.eventlog");
    let cfg = el_runtime::RuntimeConfig {
        mode: el_runtime::Mode::Replay,
        eventlog: Some(eventlog),
    };

    let mut buf: Vec<u8> = Vec::new();
    el_runtime::run_to(cfg, &mut buf)?;
    Ok(buf)
}

fn bundle() -> anyhow::Result<()> {
    let out_dir = PathBuf::from("demo/out/last_run");
    fs::create_dir_all(&out_dir)?;

    let ts = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    let demo_out = run_demo_to_bytes()?;

    let stdout_path = out_dir.join("stdout.txt");
    let runlog_path = out_dir.join("run.log");
    let manifest_path = out_dir.join("manifest.json");
    let sha_path = out_dir.join("sha256.txt");

    let mut files = std::collections::BTreeMap::new();
    files.insert(
        "stdout.txt".to_string(),
        write_file(&stdout_path, &demo_out)?,
    );
    files.insert("run.log".to_string(), write_file(&runlog_path, &demo_out)?);

    let manifest = Manifest {
        ts_utc: ts,
        app: "execution-lab".to_string(),
        cmd: vec!["execution-lab".to_string(), "demo".to_string()],
        cwd: ".".to_string(),
        files: files.clone(),
    };

    let manifest_json = serde_json::to_vec_pretty(&manifest)?;
    files.insert(
        "manifest.json".to_string(),
        write_file(&manifest_path, &manifest_json)?,
    );

    // sha256.txt (stable order)
    let mut lines: Vec<String> = Vec::new();
    for name in ["manifest.json", "stdout.txt", "run.log"] {
        let p = out_dir.join(name);
        let b = fs::read(&p)?;
        lines.push(format!("{}  {}", sha256_hex(&b), name));
    }
    let sha_txt = lines.join("\n") + "\n";
    files.insert(
        "sha256.txt".to_string(),
        write_file(&sha_path, sha_txt.as_bytes())?,
    );

    // emit minimal success markers
    println!("BUNDLE_OK");
    println!("OUT={}", out_dir.display());
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Connect { exchange } => {
            println!("app: connect OK (exchange={})", exchange);
        }
        Cmd::Load { eventlog } => {
            println!("app: load OK (eventlog={})", eventlog.display());
        }
        Cmd::Run { mode, eventlog } => {
            let cfg = match mode.as_str() {
                "replay" => el_runtime::RuntimeConfig {
                    mode: el_runtime::Mode::Replay,
                    eventlog,
                },
                "live" => el_runtime::RuntimeConfig {
                    mode: el_runtime::Mode::Live,
                    eventlog: None,
                },
                other => return Err(anyhow::anyhow!("unknown --mode={}", other)),
            };
            el_runtime::run(cfg)?;
            println!("app: run OK");
        }
        Cmd::Show => {
            println!("app: show OK");
        }
        Cmd::Demo => {
            let out = run_demo_to_bytes()?;
            std::io::stdout().write_all(&out)?;
            println!("app: demo OK");
        }
        Cmd::Bundle => {
            bundle()?;
        }
    }

    Ok(())
}
