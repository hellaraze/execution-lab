use crate::error::EventLogError;
use el_core::event::Event;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct EventLogReader {
    lines: std::io::Lines<BufReader<File>>,
}

impl EventLogReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, EventLogError> {
        let f = File::open(path.as_ref())?;
        Ok(Self {
            lines: BufReader::new(f).lines(),
        })
    }

    pub fn next(&mut self) -> Result<Option<Event>, EventLogError> {
        match self.lines.next() {
            None => Ok(None),
            Some(line) => {
                let line = line?;
                if line.trim().is_empty() {
                    return Ok(None);
                }
                let ev: Event = serde_json::from_str(&line)?;
                Ok(Some(ev))
            }
        }
    }
}
