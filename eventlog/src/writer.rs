use anyhow::{Context, Result};
use crc32fast::Hasher;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::envelope::EventEnvelope;

pub struct EventLogWriter {
    stream: String,
    next_seq: u64,
    out: BufWriter<File>,
}

impl EventLogWriter {
    // new canonical API
    pub fn open_append(path: impl AsRef<Path>, stream: impl Into<String>) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path.as_ref())
            .with_context(|| format!("open_append {:?}", path.as_ref()))?;

        Ok(Self {
            stream: stream.into(),
            next_seq: 1,
            out: BufWriter::new(file),
        })
    }

    pub fn append_json_value(&mut self, kind: &str, ts_ns: u64, payload: &serde_json::Value) -> Result<u64> {
        let payload_bytes = serde_json::to_vec(payload).context("serde_json::to_vec payload")?;

        let mut h = Hasher::new();
        h.update(&payload_bytes);
        let checksum = h.finalize();

        let seq = self.next_seq;
        self.next_seq = self.next_seq.saturating_add(1);

        let env = EventEnvelope {
            seq,
            ts_ns,
            stream: self.stream.clone(),
            kind: kind.to_string(),
            payload: payload_bytes,
            checksum,
        };

        let line = serde_json::to_string(&env).context("serde_json::to_string envelope")?;
        self.out.write_all(line.as_bytes())?;
        self.out.write_all(b"\n")?;
        Ok(seq)
    }

    pub fn flush(&mut self) -> Result<()> {
        self.out.flush().context("flush")?;
        self.out.get_ref().sync_all().context("sync_all")?;
        Ok(())
    }

    // ---- compatibility layer for existing code ----
    // old code expects: EventLogWriter::open(path) + writer.write(&Event)
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::open_append(path, "el:eventlog")
    }

    pub fn write<T: Serialize>(&mut self, ev: &T) -> Result<u64> {
        let v = serde_json::to_value(ev).context("serde_json::to_value event")?;
        self.append_json_value("event", 0, &v)
    }
}
