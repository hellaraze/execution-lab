use anyhow::{Context, Result};
use base64::Engine;
use crc32fast::Hasher as Crc32;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::envelope::EventEnvelope;
use fs2::FileExt;

fn crc32(bytes: &[u8]) -> u32 {
    let mut h = Crc32::new();
    h.update(bytes);
    h.finalize()
}

#[derive(Clone, Copy)]
pub enum Durability {
    Buffered,
    FsyncOnFlush,
    FsyncEvery { n: u64 },
}

pub struct EventLogWriter {
    #[allow(dead_code)]
    path: PathBuf,
    stream: String,
    next_seq: u64,
    out: BufWriter<File>,
    durability: Durability,
    since_fsync: u64,
}

impl EventLogWriter {
    pub fn open_append(
        path: impl AsRef<Path>,
        stream: impl Into<String>,
        durability: Durability,
    ) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&path)
            .with_context(|| format!("open {:?}", path))?;

        file.lock_exclusive().context("lock_exclusive")?;

        let last_seq = recover_tail_and_last_seq(&mut file)?;
        file.seek(SeekFrom::End(0))?;

        Ok(Self {
            path,
            stream: stream.into(),
            next_seq: last_seq + 1,
            out: BufWriter::new(file),
            durability,
            since_fsync: 0,
        })
    }

    pub fn append_bytes(&mut self, kind: &str, ts_ns: u64, payload: &[u8]) -> Result<u64> {
        let checksum = crc32(payload);
        let payload_b64 = base64::engine::general_purpose::STANDARD.encode(payload);

        let seq = self.next_seq;
        self.next_seq += 1;

        let env = EventEnvelope {
            seq,
            ts_ns,
            stream: self.stream.clone(),
            kind: kind.to_string(),
            payload_b64,
            checksum,
        };

        let line = serde_json::to_string(&env)?;
        self.out.write_all(line.as_bytes())?;
        self.out.write_all(b"\n")?;

        self.since_fsync += 1;

        if let Durability::FsyncEvery { n } = self.durability {
            if self.since_fsync >= n {
                self.flush()?;
            }
        }

        Ok(seq)
    }

    pub fn append_json_value(
        &mut self,
        kind: &str,
        ts_ns: u64,
        payload: &serde_json::Value,
    ) -> Result<u64> {
        let bytes = serde_json::to_vec(payload)?;
        self.append_bytes(kind, ts_ns, &bytes)
    }

    pub fn write<T: Serialize>(&mut self, ev: &T) -> Result<u64> {
        let bytes = serde_json::to_vec(ev)?;
        self.append_bytes("event", 0, &bytes)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::open_append(path, "el:eventlog", Durability::Buffered)
    }

    pub fn flush(&mut self) -> Result<()> {
        self.out.flush()?;
        match self.durability {
            Durability::Buffered => {}
            _ => {
                self.out.get_ref().sync_all()?;
                self.since_fsync = 0;
            }
        }
        Ok(())
    }
}

fn recover_tail_and_last_seq(file: &mut File) -> Result<u64> {
    file.seek(SeekFrom::Start(0))?;
    let mut reader = BufReader::new(&mut *file);

    let mut buf = String::new();
    let mut offset = 0u64;
    let mut last_good = 0u64;
    let mut last_seq = 0u64;

    loop {
        buf.clear();
        let n = reader.read_line(&mut buf)?;
        if n == 0 {
            break;
        }
        offset += n as u64;

        let line = buf.trim_end();
        if line.is_empty() {
            continue;
        }

        match serde_json::from_str::<EventEnvelope>(line) {
            Ok(env) => {
                last_seq = env.seq;
                last_good = offset;
            }
            Err(_) => break,
        }
    }

    drop(reader);

    let len = file.metadata()?.len();
    if last_good < len {
        file.set_len(last_good)?;
        file.sync_all().ok();
    }

    Ok(last_seq)
}
