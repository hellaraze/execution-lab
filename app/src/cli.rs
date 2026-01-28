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
    Demo,
    Replay,
    Paper,
    Live,
    Status,
    Health,
    Diagnose,
}
