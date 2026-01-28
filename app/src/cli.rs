use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "elctl", version, about = "Execution-Lab Control Plane")]
pub struct Cli {
    /// Path to config file (TOML)
    #[arg(long, default_value = "config/elctl.toml")]
    pub config: String,

    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Demo(DemoArgs),
    Replay(ReplayArgs),

    #[command(subcommand)]
    Exchange(ExchangeCmd),

    #[command(subcommand)]
    Md(MdCmd),

    Paper,
    Live,
    Status,
    Health,
    Diagnose,
}

#[derive(Subcommand)]
pub enum ExchangeCmd {
    List,
    Connect(ExchangeConnectArgs),
}

#[derive(clap::Args)]
pub struct ExchangeConnectArgs {
    pub exchange: String,
    #[arg(long)]
    pub secrets_file: String,
    #[arg(long, default_value = "evidence/connect_evidence.json")]
    pub evidence: String,
}

#[derive(Subcommand)]
pub enum MdCmd {
    List,
    Start(MdStartArgs),
}

#[derive(clap::Args)]
pub struct MdStartArgs {
    pub exchange: String,

    /// Symbol for depth stream (binance default: BTCUSDT)
    #[arg(long, default_value = "BTCUSDT")]
    pub symbol: String,

    /// Output path for NDJSON (binance_depth writes ndjson in Phase 4)
    #[arg(long, default_value = "md_out/binance_depth.ndjson")]
    pub log_path: String,

    /// Evidence JSON path
    #[arg(long, default_value = "evidence/md_start_evidence.json")]
    pub evidence: String,
}

#[derive(clap::Args)]
pub struct DemoArgs {
    #[arg(long)]
    pub input: String,
    #[arg(long, default_value = "evidence/demo_evidence.json")]
    pub evidence: String,
    #[arg(long, default_value_t = 20)]
    pub top_n: u32,
}

#[derive(clap::Args)]
pub struct ReplayArgs {
    #[arg(long)]
    pub input: String,
    #[arg(long, default_value = "evidence/replay_evidence.json")]
    pub evidence: String,
    #[arg(long, default_value_t = 20)]
    pub top_n: u32,
}
