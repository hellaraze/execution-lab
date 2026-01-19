use crate::error::EventLogError;
use el_core::event::Event;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct EventLogWriter {
    w: BufWriter<std::fs::File>,
    n: u64,
}

impl EventLogWriter {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, EventLogError> {
        let f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Self { w: BufWriter::new(f), n: 0 })
    }

    pub fn write(&mut self, ev: &Event) -> Result<(), EventLogError> {
        serde_json::to_writer(&mut self.w, ev)?;
        self.w.write_all(b"\n")?;
        self.w.flush()?; // для дебага, потом можно убрать
        self.n += 1;

        if self.n % 100 == 0 {
            eprintln!("[eventlog] wrote {} events", self.n);
        }
        Ok(())
    }
}
