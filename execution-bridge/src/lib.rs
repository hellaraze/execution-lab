use anyhow::Result;
use el_core::event::{Event, EventId};
use eventlog::EventLogWriter;
use serde_json;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(pub EventId);

pub trait ExecOutbox {
    fn publish_once(&mut self, ev: Event) -> Result<()>;
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
    fn publish_once(&mut self, ev: Event) -> Result<()> {
        // ЖБ-контракт:
        // - writer.append MUST be idempotent by EventId
        // - duplicate EventId -> NO-OP
        
        let payload = serde_json::to_vec(&ev)?;
        let kind = "event";
        let ts_ns: u64 = ev.ts_proc.0;
        self.writer.append_bytes(kind, ts_ns, &payload)?;

        Ok(())
    }
}
