use crate::error::EventLogError;
use el_core::event::Event;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct EventLogReader {
    reader: BufReader<File>,
}

impl EventLogReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, EventLogError> {
        let file =
            File::open(path.as_ref()).map_err(|e| EventLogError::Io(e.to_string()))?;

        Ok(Self {
            reader: BufReader::new(file),
        })
    }

    pub fn next(&mut self) -> Result<Option<Event>, EventLogError> {
        let mut line = String::new();
        let n = self
            .reader
            .read_line(&mut line)
            .map_err(|e| EventLogError::Io(e.to_string()))?;

        if n == 0 {
            return Ok(None);
        }

        let event: Event =
            serde_json::from_str(&line).map_err(|_| EventLogError::CorruptedEntry)?;

        Ok(Some(event))
    }
}
