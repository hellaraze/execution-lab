use std::process::{Command, Stdio};

pub struct RunOut {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn run_cmd(mut cmd: Command) -> anyhow::Result<RunOut> {
    let out = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output()?;
    let code = out.status.code().unwrap_or(1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    Ok(RunOut {
        exit_code: code,
        stdout,
        stderr,
    })
}
