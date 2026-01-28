use serde::Serialize;

#[derive(Serialize)]
pub struct StatusOut {
    pub ok: bool,
    pub baseline_tag: &'static str,
    pub git_head: Option<String>,
    pub mode: String,
}

#[derive(Serialize)]
pub struct HealthOut {
    pub ok: bool,
}

#[derive(Serialize)]
pub struct DiagnoseOut {
    pub ok: bool,
    pub notes: Vec<String>,
}

pub fn git_head() -> Option<String> {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8(out.stdout).ok()?;
    Some(s.trim().to_string())
}
