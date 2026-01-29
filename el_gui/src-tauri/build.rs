use std::process::Command;

fn main() {
  // git sha (best-effort)
  let sha = Command::new("git")
    .args(["rev-parse", "--short=12", "HEAD"])
    .output()
    .ok()
    .and_then(|o| if o.status.success() { Some(String::from_utf8_lossy(&o.stdout).trim().to_string()) } else { None })
    .unwrap_or_else(|| "unknown".to_string());

  // build time UTC
  let ts = Command::new("date")
    .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
    .output()
    .ok()
    .and_then(|o| if o.status.success() { Some(String::from_utf8_lossy(&o.stdout).trim().to_string()) } else { None })
    .unwrap_or_else(|| "unknown".to_string());

  println!("cargo:rustc-env=GIT_SHA={sha}");
  println!("cargo:rustc-env=BUILD_TIME_UTC={ts}");
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=tauri.conf.json");

  tauri_build::build();
}
