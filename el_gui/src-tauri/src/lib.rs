#![cfg_attr(mobile, tauri::mobile_entry_point)]

use serde::Serialize;
use std::{path::PathBuf, sync::Arc};
use tokio::{process::Child, sync::Mutex};

#[derive(Clone)]
struct AppState {
  child: Arc<Mutex<Option<Child>>>,
  log_path: PathBuf,
  root: PathBuf,
}

fn repo_root_from_manifest_dir() -> PathBuf {
  let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  // el_gui/src-tauri -> el_gui -> repo root
  p.pop();
  p.pop();
  p
}

#[derive(Serialize)]
struct AppInfo {
  version: String,
  git_sha: String,
  build_time_utc: String,
  bundle_id: String,
}

#[tauri::command]
fn app_info() -> AppInfo {
  AppInfo {
    version: env!("CARGO_PKG_VERSION").to_string(),
    git_sha: option_env!("GIT_SHA").unwrap_or("unknown").to_string(),
    build_time_utc: option_env!("BUILD_TIME_UTC").unwrap_or("unknown").to_string(),
    bundle_id: "com.hellaraze.executionlab".to_string(),
  }
}

#[tauri::command]
async fn runtime_status(state: tauri::State<'_, AppState>) -> Result<String, String> {
  let g = state.child.lock().await;
  Ok(if g.is_some() { "running".to_string() } else { "idle".to_string() })
}

#[tauri::command]
async fn runtime_stop(state: tauri::State<'_, AppState>) -> Result<String, String> {
  let mut g = state.child.lock().await;
  if let Some(mut c) = g.take() {
    let _ = c.kill().await;
    Ok("stopped".to_string())
  } else {
    Ok("idle".to_string())
  }
}

#[tauri::command]
async fn runtime_start_level3_gate(state: tauri::State<'_, AppState>) -> Result<String, String> {
  {
    let g = state.child.lock().await;
    if g.is_some() {
      return Err("already running".to_string());
    }
  }

  std::fs::write(&state.log_path, "").map_err(|e| format!("log truncate failed: {e}"))?;

  let mut cmd = tokio::process::Command::new("bash");
  cmd.arg(state.root.join("tools").join("level3_gate.sh"));

  let f = std::fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open(&state.log_path)
    .map_err(|e| format!("log open failed: {e}"))?;
  let f2 = f.try_clone().map_err(|e| format!("log clone failed: {e}"))?;
  cmd.stdout(f);
  cmd.stderr(f2);

  let child = cmd.spawn().map_err(|e| format!("spawn failed: {e}"))?;
  let mut g = state.child.lock().await;
  *g = Some(child);

  Ok("started: level3_gate".to_string())
}

#[tauri::command]
async fn runtime_log_tail(
  state: tauri::State<'_, AppState>,
  max_chars: Option<usize>,
) -> Result<String, String> {
  let s = std::fs::read_to_string(&state.log_path).unwrap_or_else(|_| "".to_string());
  let n = max_chars.unwrap_or(20000);
  if s.len() <= n {
    return Ok(s);
  }
  Ok(s[s.len() - n..].to_string())
}

pub fn run() {
  let root = repo_root_from_manifest_dir();
  let log_path = root.join("el_gui").join("runtime_bridge.log");

  let state = AppState {
    child: Arc::new(Mutex::new(None)),
    log_path,
    root,
  };

  tauri::Builder::default()
    .manage(state)
    .plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())
    .invoke_handler(tauri::generate_handler![
      app_info,
      runtime_status,
      runtime_start_level3_gate,
      runtime_stop,
      runtime_log_tail
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
