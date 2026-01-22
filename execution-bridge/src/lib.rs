use anyhow::Result;
use el_core::event::{ExecEvent, EventId};
use eventlog::EventLogWriter;
use serde_json;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(pub EventId);

pub trait ExecOutbox {
    fn publish_once(&mut self, ev: ExecEvent) -> Result<()>;
}

pub struct Bridge {
    writer: EventLogWriter,
}

impl Bridge {
    pub fn new(writer: EventLogWriter) -> Self {
        Self { writer }
    }
}

impl ExecOutbox for Bridge {
    fn publish_once(&mut self, ev: ExecEvent) -> Result<()> {
        // ЖБ-контракт:
        // - writer.append MUST be idempotent by EventId
        // - duplicate EventId -> NO-OP
        
        let payload = serde_json::to_vec(&ev)?;
        let kind = "exec";
        let ts_ns: u64 = ev.ts_ns();
        self.writer.append_bytes(kind, ts_ns, &payload)?;

        Ok(())
    }
}
