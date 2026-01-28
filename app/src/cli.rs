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
    /// Deterministic demo run (offline)
    Demo(DemoArgs),

    /// Replay run (offline)
    Replay(ReplayArgs),

    /// Paper mode (disabled in Phase 2)
    Paper,

    /// Live mode (hard-disabled)
    Live,

    /// System status (JSON)
    Status,

    /// System health (JSON)
    Health,

    /// Diagnostics (JSON)
    Diagnose,
}

#[derive(clap::Args)]
pub struct DemoArgs {
    /// Path to input eventlog
    #[arg(long)]
    pub input: String,

    /// Where to write evidence JSON
    #[arg(long, default_value = "evidence/demo_evidence.json")]
    pub evidence: String,

    /// How many rows to request from D2 scan
    #[arg(long, default_value_t = 20)]
    pub top_n: u32,
}

#[derive(clap::Args)]
pub struct ReplayArgs {
    /// Path to input eventlog
    #[arg(long)]
    pub input: String,

    /// Where to write evidence JSON
    #[arg(long, default_value = "evidence/replay_evidence.json")]
    pub evidence: String,

    /// How many rows to request from D2 scan
    #[arg(long, default_value_t = 20)]
    pub top_n: u32,
}
