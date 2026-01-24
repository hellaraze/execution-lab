use anyhow::{Context, Result};
use crc32fast::Hasher;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::envelope::EventEnvelope;
use base64::Engine;

fn crc32(bytes: &[u8]) -> u32 {
    let mut h = Hasher::new();
    h.update(bytes);
    h.finalize()
}

pub struct EventLogReader {
    r: BufReader<File>,
    line_buf: String,
}

impl EventLogReader {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let file =
            File::open(path.as_ref()).with_context(|| format!("open {:?}", path.as_ref()))?;
        Ok(Self {
            r: BufReader::new(file),
            line_buf: String::new(),
        })
    }

    pub fn read_next(&mut self) -> Result<Option<(EventEnvelope, Vec<u8>)>> {
        self.line_buf.clear();
        let n = self.r.read_line(&mut self.line_buf)?;
        if n == 0 {
            return Ok(None);
        }

        let env: EventEnvelope =
            serde_json::from_str(self.line_buf.trim_end()).context("parse envelope json")?;

        let payload = base64::engine::general_purpose::STANDARD
            .decode(&env.payload_b64)
            .context("base64 decode payload")?;

        let checksum = crc32(&payload);
        if checksum != env.checksum {
            anyhow::bail!("checksum mismatch: seq={}", env.seq);
        }

        Ok(Some((env, payload)))
    }
}
