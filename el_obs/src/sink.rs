use super::event::ObsEvent;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

pub trait ObsSink {
    fn emit(&mut self, ev: ObsEvent);
}

pub struct NoopSink;

impl ObsSink for NoopSink {
    fn emit(&mut self, _ev: ObsEvent) {}
}

/// Deterministic JSONL sink: one event per line (serde_json), append-only.
pub struct FileSink {
    w: BufWriter<File>,
}

impl FileSink {
    pub fn open_append(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let f = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { w: BufWriter::new(f) })
    }
}

impl ObsSink for FileSink {
    fn emit(&mut self, ev: ObsEvent) {
        let line = serde_json::to_string(&ev).expect("ObsEvent serialize");
        self.w.write_all(line.as_bytes()).expect("FileSink write");
        self.w.write_all(b"\n").expect("FileSink newline");
        self.w.flush().expect("FileSink flush");
    }
}
