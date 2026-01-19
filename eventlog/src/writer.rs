use crate::error::EventLogError;
use el_core::event::Event;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct EventLogWriter {
    writer: BufWriter<File>,
}

impl EventLogWriter {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, EventLogError> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path.as_ref())
            .map_err(|e| EventLogError::Io(e.to_string()))?;

        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    pub fn append(&mut self, event: &Event) -> Result<(), EventLogError> {
        let line =
            serde_json::to_string(event).map_err(|e| EventLogError::Serde(e.to_string()))?;

        self.writer
            .write_all(line.as_bytes())
            .map_err(|e| EventLogError::Io(e.to_string()))?;

        self.writer
            .write_all(b"\n")
            .map_err(|e| EventLogError::Io(e.to_string()))?;

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), EventLogError> {
        self.writer
            .flush()
            .map_err(|e| EventLogError::Io(e.to_string()))
    }
}
