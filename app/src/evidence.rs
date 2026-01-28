use serde::Serialize;

#[derive(Serialize)]
pub struct Evidence {
    pub ok: bool,
    pub baseline_tag: &'static str,
    pub git_head: Option<String>,
    pub mode: String,
    pub input: String,
    pub tool: ToolRun,
}

#[derive(Serialize)]
pub struct ToolRun {
    pub cmd: Vec<String>,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn write_json(path: &str, ev: &Evidence) -> anyhow::Result<()> {
    let dir = std::path::Path::new(path)
        .parent()
        .unwrap_or(std::path::Path::new("."));
    std::fs::create_dir_all(dir)?;
    let s = serde_json::to_string_pretty(ev)?;
    std::fs::write(path, s)?;
    Ok(())
}
