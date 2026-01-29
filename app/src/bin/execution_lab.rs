use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "execution-lab",
    version,
    about = "Execution-Lab Product Runtime (single entrypoint)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Run system using a product config (replay/live)
    Run {
        #[arg(long)]
        config: PathBuf,
    },

    /// Validate config and exit (fail-fast)
    ValidateConfig {
        #[arg(long)]
        config: PathBuf,
    },

    /// Write a sample config
    InitConfig {
        #[arg(long)]
        out: PathBuf,
        #[arg(long, default_value = "replay")]
        kind: String,
    },
}

fn real_main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Run { config } => {
            let (id, dir) = el_runtime::product::run_from_config_path(&config)?;
            println!("RUN_OK");
            println!("RUN_ID={}", id);
            println!("RUN_DIR={}", dir.display());
        }
        Cmd::ValidateConfig { config } => {
            el_runtime::product::validate_config(&config)?;
            println!("VALID_OK");
        }
        Cmd::InitConfig { out, kind } => {
            el_runtime::product::init_config(&out, &kind)?;
            println!("INIT_OK");
            println!("OUT={}", out.display());
        }
    }
    Ok(())
}

fn main() -> std::process::ExitCode {
    if let Err(e) = real_main() {
        eprintln!("ERROR: {:#}", e);
        std::process::ExitCode::from(2)
    } else {
        std::process::ExitCode::SUCCESS
    }
}
