use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: u32,
    pub mode: Mode,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Demo,
    Replay,
    Paper,
    Live,
}

pub fn load(path: &str) -> anyhow::Result<Config> {
    let raw = std::fs::read_to_string(path).with_context(|| format!("read config: {}", path))?;
    let cfg: Config = toml::from_str(&raw).with_context(|| format!("parse toml: {}", path))?;
    if cfg.version != 1 {
        anyhow::bail!("unsupported config version: {} (expected 1)", cfg.version);
    }
    Ok(cfg)
}
