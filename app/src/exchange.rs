use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Capabilities {
    pub spot: bool,
    pub perp: bool,
    pub sandbox: bool,

    // Market data (Phase 4)
    pub md_depth: bool,
    pub md_bbo: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExchangeInfo {
    pub name: &'static str,
    pub id: &'static str,
    pub capabilities: Capabilities,
    pub required_secrets: &'static [&'static str],
}

pub fn registry() -> Vec<ExchangeInfo> {
    vec![
        ExchangeInfo {
            name: "Binance",
            id: "binance",
            capabilities: Capabilities {
                spot: true,
                perp: true,
                sandbox: false,
                md_depth: true,
                md_bbo: true,
            },
            required_secrets: &["api_key", "api_secret"],
        },
        ExchangeInfo {
            name: "OKX",
            id: "okx",
            capabilities: Capabilities {
                spot: true,
                perp: true,
                sandbox: true,
                md_depth: true,
                md_bbo: true,
            },
            required_secrets: &["api_key", "api_secret", "passphrase"],
        },
        ExchangeInfo {
            name: "Bybit",
            id: "bybit",
            capabilities: Capabilities {
                spot: true,
                perp: true,
                sandbox: true,
                md_depth: true,
                md_bbo: true,
            },
            required_secrets: &["api_key", "api_secret"],
        },
    ]
}

pub fn find_exchange(id: &str) -> Option<ExchangeInfo> {
    registry()
        .into_iter()
        .find(|e| e.id.eq_ignore_ascii_case(id))
}

#[derive(Debug, Deserialize)]
pub struct SecretsFile {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub passphrase: Option<String>,
}

impl SecretsFile {
    pub fn missing_for(&self, required: &[&str]) -> Vec<String> {
        let mut missing = Vec::new();
        for k in required {
            let ok = match *k {
                "api_key" => self
                    .api_key
                    .as_ref()
                    .map(|v| !v.is_empty())
                    .unwrap_or(false),
                "api_secret" => self
                    .api_secret
                    .as_ref()
                    .map(|v| !v.is_empty())
                    .unwrap_or(false),
                "passphrase" => self
                    .passphrase
                    .as_ref()
                    .map(|v| !v.is_empty())
                    .unwrap_or(false),
                _ => false,
            };
            if !ok {
                missing.push(k.to_string());
            }
        }
        missing
    }
}

#[derive(Serialize)]
pub struct ConnectResult {
    pub ok: bool,
    pub exchange: String,
    pub secrets_present: Vec<String>,
    pub secrets_missing: Vec<String>,
    pub checks: Vec<CheckResult>,
}

#[derive(Serialize)]
pub struct CheckResult {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

pub fn load_secrets_toml(path: &str) -> anyhow::Result<SecretsFile> {
    let raw = std::fs::read_to_string(path)?;
    let s: SecretsFile = toml::from_str(&raw)?;
    Ok(s)
}
