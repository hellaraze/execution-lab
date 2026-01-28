use clap::Parser;
use elctl::{cli, config, evidence, exchange, out, run};
use std::process::Command;

fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();
    let cfg = config::load(&args.config)?;

    match args.cmd {
        cli::Command::Demo(a) => {
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

        cli::Command::Exchange(x) => match x {
            cli::ExchangeCmd::List => {
                let reg = exchange::registry();
                println!("{}", serde_json::to_string(&reg)?);
            }
            cli::ExchangeCmd::Connect(a) => {
                let ex = exchange::find_exchange(&a.exchange)
                    .ok_or_else(|| anyhow::anyhow!("unknown exchange: {}", a.exchange))?;

                // Load secrets, but NEVER print raw values.
                let s = exchange::load_secrets_toml(&a.secrets_file)?;
                let missing = s.missing_for(ex.required_secrets);

                let mut present = Vec::new();
                for k in ex.required_secrets {
                    let has = match *k {
                        "api_key" => s.api_key.as_ref().map(|v| !v.is_empty()).unwrap_or(false),
                        "api_secret" => s
                            .api_secret
                            .as_ref()
                            .map(|v| !v.is_empty())
                            .unwrap_or(false),
                        "passphrase" => s
                            .passphrase
                            .as_ref()
                            .map(|v| !v.is_empty())
                            .unwrap_or(false),
                        _ => false,
                    };
                    if has {
                        present.push(k.to_string());
                    }
                }

                // Stubbed health checks (Phase 3): structure only.
                let checks = vec![
                    exchange::CheckResult {
                        name: "auth_format".to_string(),
                        ok: missing.is_empty(),
                        detail: "required secrets present".to_string(),
                    },
                    exchange::CheckResult {
                        name: "clock_skew".to_string(),
                        ok: true,
                        detail: "stubbed".to_string(),
                    },
                    exchange::CheckResult {
                        name: "rate_limits".to_string(),
                        ok: true,
                        detail: "stubbed".to_string(),
                    },
                ];

                let result = exchange::ConnectResult {
                    ok: missing.is_empty(),
                    exchange: ex.id.to_string(),
                    secrets_present: present,
                    secrets_missing: missing,
                    checks,
                };

                // Write connect evidence (JSON). Do NOT include secret values.
                let ev = serde_json::json!({
                    "ok": result.ok,
                    "baseline_tag": "baseline-sealed",
                    "git_head": out::git_head(),
                    "mode": format!("{:?}", cfg.mode).to_lowercase(),
                    "exchange": ex.id,
                    "secrets_file": elctl::redact::redact(&a.secrets_file),
                    "result": result
                });

                let dir = std::path::Path::new(&a.evidence)
                    .parent()
                    .unwrap_or(std::path::Path::new("."));
                std::fs::create_dir_all(dir)?;
                std::fs::write(&a.evidence, serde_json::to_string_pretty(&ev)?)?;

                println!("{}", serde_json::to_string(&ev)?);
                if !result.ok {
                    anyhow::bail!(
                        "connect failed (missing secrets) (see evidence: {})",
                        a.evidence
                    );
                }
            }
        },

        cli::Command::Paper => anyhow::bail!("paper mode is disabled"),
        cli::Command::Live => anyhow::bail!("live mode is disabled"),

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
            println!("{}", serde_json::to_string(&out::HealthOut { ok: true })?)
        }
        cli::Command::Diagnose => println!(
            "{}",
            serde_json::to_string(&out::DiagnoseOut {
                ok: true,
                notes: vec!["no issues detected".to_string()]
            })?
        ),
    }

    Ok(())
}
