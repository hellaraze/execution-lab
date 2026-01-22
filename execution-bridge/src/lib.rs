use anyhow::Result;
use el_core::event::ExecEvent;
use eventlog::{EventLogWriter, EventId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(pub EventId);

pub trait ExecOutbox {
    fn publish_once(&mut self, ev: ExecEvent) -> Result<()>;
}

pub struct Bridge<W: EventLogWriter> {
    writer: W,
}

impl<W: EventLogWriter> Bridge<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: EventLogWriter> ExecOutbox for Bridge<W> {
    fn publish_once(&mut self, ev: ExecEvent) -> Result<()> {
        // ЖБ-контракт:
        // - writer.append MUST be idempotent by EventId
        // - duplicate EventId -> NO-OP
        self.writer.append(ev)?;
        Ok(())
    }
}
