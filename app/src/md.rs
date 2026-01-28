use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
pub struct MdListOut {
    pub exchanges: Vec<MdExchange>,
}

#[derive(Serialize)]
pub struct MdExchange {
    pub id: String,
    pub name: String,
    pub md_depth: bool,
    pub md_bbo: bool,
}

#[derive(Serialize)]
pub struct MdStartOut {
    pub ok: bool,
    pub exchange: String,
    pub symbol: String,
    pub log_path: String,
    pub cmd: Vec<String>,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn md_list() -> MdListOut {
    let reg = crate::exchange::registry();
    let exchanges = reg
        .into_iter()
        .map(|e| MdExchange {
            id: e.id.to_string(),
            name: e.name.to_string(),
            md_depth: e.capabilities.md_depth,
            md_bbo: e.capabilities.md_bbo,
        })
        .collect();
    MdListOut { exchanges }
}

pub fn start_binance_depth(symbol: &str, log_path: &str) -> anyhow::Result<MdStartOut> {
    // connectors/binance_depth positional args:
    //   1) symbol (default BTCUSDT)
    //   2) log_path (default /tmp/binance_depth.ndjson)
    let mut c = Command::new("timeout");
    c.args(["10s", "cargo"]);
    c.args([
        "run",
        "-q",
        "-p",
        "connectors",
        "--bin",
        "binance_depth",
        "--",
        symbol,
        log_path,
    ]);

    let cmd = vec![
        "cargo".to_string(),
        "run".to_string(),
        "-q".to_string(),
        "-p".to_string(),
        "connectors".to_string(),
        "--bin".to_string(),
        "binance_depth".to_string(),
        "--".to_string(),
        symbol.to_string(),
        log_path.to_string(),
    ];

    let r = crate::run::run_cmd(c)?;
    Ok(MdStartOut {
        ok: r.exit_code == 0,
        exchange: "binance".to_string(),
        symbol: symbol.to_string(),
        log_path: log_path.to_string(),
        cmd,
        exit_code: r.exit_code,
        stdout: r.stdout,
        stderr: r.stderr,
    })
}
