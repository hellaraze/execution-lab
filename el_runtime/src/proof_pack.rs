use anyhow::Context;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Builder;

#[derive(Debug, Serialize)]
pub struct FileEntry {
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Debug, Serialize)]
pub struct ProofManifest {
    pub ts_utc: String,
    pub run_id: String,
    pub git_head: String,
    pub files: BTreeMap<String, FileEntry>,
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn write_file(path: &Path, bytes: &[u8]) -> anyhow::Result<FileEntry> {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p)?;
    }
    fs::write(path, bytes)?;
    let meta = fs::metadata(path)?;
    Ok(FileEntry {
        sha256: sha256_hex(bytes),
        bytes: meta.len(),
    })
}

fn cmd_out(args: &[&str]) -> String {
    if args.is_empty() {
        return "ERR(empty)".to_string();
    }
    let mut it = args.iter();
    let prog = it.next().unwrap();
    let out = Command::new(prog).args(it).output();
    match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Ok(o) => format!(
            "ERR(exit={}): {}",
            o.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&o.stderr).trim()
        ),
        Err(e) => format!("ERR(exec): {e}"),
    }
}

fn git_head() -> String {
    cmd_out(&["git", "rev-parse", "HEAD"])
}

pub fn create_proof_pack(run_dir: &Path) -> anyhow::Result<PathBuf> {
    let run_id = run_dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "UNKNOWN_RUN".to_string());

    if !run_dir.exists() {
        anyhow::bail!("run_dir does not exist: {}", run_dir.display());
    }

    let proof_dir = run_dir.join("proof");
    fs::create_dir_all(&proof_dir)?;

    // toolchain/env
    let toolchain_txt = format!(
        "rustc: {}\ncargo: {}\n",
        cmd_out(&["rustc", "-V"]),
        cmd_out(&["cargo", "-V"])
    );
    let env_txt = format!(
        "uname: {}\nhead: {}\n",
        cmd_out(&["uname", "-a"]),
        git_head()
    );

    let mut files: BTreeMap<String, FileEntry> = BTreeMap::new();
    files.insert(
        "proof/toolchain.txt".to_string(),
        write_file(&proof_dir.join("toolchain.txt"), toolchain_txt.as_bytes())?,
    );
    files.insert(
        "proof/env.txt".to_string(),
        write_file(&proof_dir.join("env.txt"), env_txt.as_bytes())?,
    );

    // Create TAR.GZ OUTSIDE run_dir first, then move in (avoid self-inclusion)
    let parent = run_dir.parent().unwrap_or(Path::new("."));
    let tmp_pack = parent.join(format!("{run_id}.proof_pack.tar.gz.tmp"));
    let final_pack = run_dir.join("proof_pack.tar.gz");

    // build tar.gz
    let f = File::create(&tmp_pack)
        .with_context(|| format!("create tmp pack: {}", tmp_pack.display()))?;
    let enc = GzEncoder::new(f, Compression::default());
    let mut tar = Builder::new(enc);

    // include entire run_dir as "run/<run_id>/..."
    tar.append_dir_all(format!("run/{run_id}"), run_dir)
        .with_context(|| format!("tar append run_dir: {}", run_dir.display()))?;

    let enc = tar.into_inner().context("tar into_inner")?;
    enc.finish().context("gz finish")?;

    // move into run_dir
    if final_pack.exists() {
        fs::remove_file(&final_pack).ok();
    }
    fs::rename(&tmp_pack, &final_pack)
        .with_context(|| format!("rename pack to: {}", final_pack.display()))?;

    // hash pack
    let pack_bytes = fs::read(&final_pack)?;
    files.insert(
        "proof_pack.tar.gz".to_string(),
        FileEntry {
            sha256: sha256_hex(&pack_bytes),
            bytes: pack_bytes.len() as u64,
        },
    );

    // manifest
    let ts = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let m = ProofManifest {
        ts_utc: ts,
        run_id,
        git_head: git_head(),
        files,
    };
    let manifest_json = serde_json::to_vec_pretty(&m)?;
    // store manifest inside proof/
    let mf_entry = write_file(&proof_dir.join("proof_manifest.json"), &manifest_json)?;
    // (not strictly needed, but keeps manifest self-described)
    let mut mf_map: BTreeMap<String, FileEntry> = BTreeMap::new();
    mf_map.insert("proof/proof_manifest.json".to_string(), mf_entry);
    let _ = mf_map;

    Ok(final_pack)
}
