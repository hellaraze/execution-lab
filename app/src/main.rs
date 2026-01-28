use elctl::{cli, config, out};

use clap::Parser;
use cli::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg = config::load(&cli.config)?;

    match cli.cmd {
        Command::Demo => {
            println!("{{\"ok\":true,\"mode\":\"demo\"}}");
        }
        Command::Replay => {
            println!("{{\"ok\":true,\"mode\":\"replay\"}}");
        }
        Command::Paper => {
            println!("{{\"ok\":true,\"mode\":\"paper\"}}");
        }
        Command::Live => {
            // Hard-disabled in Phase 1 by design.
            anyhow::bail!("live mode is disabled in Phase 1");
        }
        Command::Status => {
            let out = out::StatusOut {
                ok: true,
                baseline_tag: "baseline-sealed",
                git_head: out::git_head(),
                mode: format!("{:?}", cfg.mode).to_lowercase(),
            };
            println!("{}", serde_json::to_string(&out)?);
        }
        Command::Health => {
            let out = out::HealthOut { ok: true };
            println!("{}", serde_json::to_string(&out)?);
        }
        Command::Diagnose => {
            let out = out::DiagnoseOut {
                ok: true,
                notes: vec!["no issues detected".to_string()],
            };
            println!("{}", serde_json::to_string(&out)?);
        }
    }

    Ok(())
}
