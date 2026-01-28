use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Mode {
    Replay,
    Live,
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub mode: Mode,
    pub eventlog: Option<PathBuf>,
}

pub fn run(cfg: RuntimeConfig) -> anyhow::Result<()> {
    let mut out = std::io::stdout();
    run_to(cfg, &mut out)
}

pub fn run_to(mut cfg: RuntimeConfig, out: &mut dyn Write) -> anyhow::Result<()> {
    match cfg.mode {
        Mode::Replay => {
            let p = cfg
                .eventlog
                .take()
                .ok_or_else(|| anyhow::anyhow!("missing --eventlog"))?;
            writeln!(out, "el_runtime: replay OK (eventlog={})", p.display())?;
        }
        Mode::Live => {
            writeln!(out, "el_runtime: live OK")?;
        }
    }
    Ok(())
}
