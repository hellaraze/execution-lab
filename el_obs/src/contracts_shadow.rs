use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::Result;
use el_contracts::v1;

/// Writes contract-shadow events as JSONL.
/// This is intentionally side-band and must never affect trading logic.
pub struct ContractShadowWriter {
    w: BufWriter<File>,
}

impl ContractShadowWriter {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let f = File::create(path)?;
        Ok(Self {
            w: BufWriter::new(f),
        })
    }

    #[inline]
    pub fn write_md(&mut self, e: &v1::md::MdEvent) -> Result<()> {
        self.write_any("md", e)
    }

    #[inline]
    pub fn write_strategy(&mut self, e: &v1::strategy::StrategyDecision) -> Result<()> {
        self.write_any("strategy", e)
    }

    #[inline]
    pub fn write_audit(&mut self, e: &v1::audit::AuditEvent) -> Result<()> {
        self.write_any("audit", e)
    }

    fn write_any<T: serde::Serialize>(&mut self, kind: &'static str, e: &T) -> Result<()> {
        // format: {"kind":"md", ...payload...}\n
        let mut obj = serde_json::Map::new();
        obj.insert("kind".into(), serde_json::Value::String(kind.into()));
        let payload = serde_json::to_value(e)?;
        match payload {
            serde_json::Value::Object(m) => {
                for (k, v) in m {
                    obj.insert(k, v);
                }
            }
            _ => {
                obj.insert("payload".into(), payload);
            }
        }
        let line = serde_json::Value::Object(obj);
        serde_json::to_writer(&mut self.w, &line)?;
        self.w.write_all(b"\n")?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.w.flush()?;
        Ok(())
    }
}
