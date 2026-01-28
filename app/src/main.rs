use clap::Parser;
use elctl::{cli, config, evidence, out, run};
use std::process::Command;

fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();
    let cfg = config::load(&args.config)?;

    match args.cmd {
        cli::Command::Demo(a) => {
            // Deterministic offline demo: use d2_scan replay-ro against an eventlog.
            let mut c = Command::new("cargo");
            c.args([
                "run",
                "-q",
                "-p",
                "d2",
                "--features",
                "replay-ro",
                "--bin",
                "d2_scan",
                "--",
                &a.input,
                "--top-n",
                &a.top_n.to_string(),
            ]);

            let cmd_vec = vec![
                "cargo".to_string(),
                "run".to_string(),
                "-q".to_string(),
                "-p".to_string(),
                "d2".to_string(),
                "--features".to_string(),
                "replay-ro".to_string(),
                "--bin".to_string(),
                "d2_scan".to_string(),
                "--".to_string(),
                a.input.clone(),
                "--top-n".to_string(),
                a.top_n.to_string(),
            ];

            let r = run::run_cmd(c)?;
            let ev = evidence::Evidence {
                ok: r.exit_code == 0,
                baseline_tag: "baseline-sealed",
                git_head: out::git_head(),
                mode: format!("{:?}", cfg.mode).to_lowercase(),
                input: a.input.clone(),
                tool: evidence::ToolRun {
                    cmd: cmd_vec,
                    exit_code: r.exit_code,
                    stdout: r.stdout,
                    stderr: r.stderr,
                },
            };
            evidence::write_json(&a.evidence, &ev)?;
            println!("{}", serde_json::to_string(&ev)?);
            if !ev.ok {
                anyhow::bail!("demo failed (see evidence: {})", a.evidence);
            }
        }

        cli::Command::Replay(a) => {
            // Replay mode is identical to demo for Phase 2 (offline, deterministic).
            let mut c = Command::new("cargo");
            c.args([
                "run",
                "-q",
                "-p",
                "d2",
                "--features",
                "replay-ro",
                "--bin",
                "d2_scan",
                "--",
                &a.input,
                "--top-n",
                &a.top_n.to_string(),
            ]);

            let cmd_vec = vec![
                "cargo".to_string(),
                "run".to_string(),
                "-q".to_string(),
                "-p".to_string(),
                "d2".to_string(),
                "--features".to_string(),
                "replay-ro".to_string(),
                "--bin".to_string(),
                "d2_scan".to_string(),
                "--".to_string(),
                a.input.clone(),
                "--top-n".to_string(),
                a.top_n.to_string(),
            ];

            let r = run::run_cmd(c)?;
            let ev = evidence::Evidence {
                ok: r.exit_code == 0,
                baseline_tag: "baseline-sealed",
                git_head: out::git_head(),
                mode: format!("{:?}", cfg.mode).to_lowercase(),
                input: a.input.clone(),
                tool: evidence::ToolRun {
                    cmd: cmd_vec,
                    exit_code: r.exit_code,
                    stdout: r.stdout,
                    stderr: r.stderr,
                },
            };
            evidence::write_json(&a.evidence, &ev)?;
            println!("{}", serde_json::to_string(&ev)?);
            if !ev.ok {
                anyhow::bail!("replay failed (see evidence: {})", a.evidence);
            }
        }

        cli::Command::Paper => {
            anyhow::bail!("paper mode is disabled in Phase 2");
        }
        cli::Command::Live => {
            anyhow::bail!("live mode is disabled");
        }
        cli::Command::Status => {
            let outj = out::StatusOut {
                ok: true,
                baseline_tag: "baseline-sealed",
                git_head: out::git_head(),
                mode: format!("{:?}", cfg.mode).to_lowercase(),
            };
            println!("{}", serde_json::to_string(&outj)?);
        }
        cli::Command::Health => {
            let outj = out::HealthOut { ok: true };
            println!("{}", serde_json::to_string(&outj)?);
        }
        cli::Command::Diagnose => {
            let outj = out::DiagnoseOut {
                ok: true,
                notes: vec!["no issues detected".to_string()],
            };
            println!("{}", serde_json::to_string(&outj)?);
        }
    }

    Ok(())
}
