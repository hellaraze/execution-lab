use anyhow::Result;
use el_core::event::{Event, EventId};
use eventlog::EventLogWriter;
use serde_json;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(pub EventId);

pub trait ExecOutbox {
    fn publish_once(&mut self, ev: Event) -> Result<()>;
}

pub struct Bridge {
    writer: EventLogWriter,
    seen: HashSet<EventId>,
}


impl Bridge {
    pub fn new(writer: EventLogWriter) -> Self {
        Self { writer, seen: HashSet::new() }
    }
    pub fn open_dedup(
        path: impl AsRef<std::path::Path>,
        stream: &str,
        durability: eventlog::writer::Durability,
    ) -> anyhow::Result<Self> {
        let path = path.as_ref();

        // replay existing log to seed seen
        let mut seen: std::collections::HashSet<el_core::event::EventId> =
            std::collections::HashSet::new();

        if path.exists() {
            let mut r = eventlog::EventLogReader::open(path)?;
            loop {
                let (env, payload) = match r.next()? {
                    Some(v) => v,
                    None => break,
                };
                if env.kind != "event" {
                    continue;
                }
                let e: el_core::event::Event = serde_json::from_slice(&payload)?;
                seen.insert(e.id);
            }
        }

        let writer = eventlog::EventLogWriter::open_append(path, stream.to_string(), durability)?;
        Ok(Self { writer, seen })
    }
}

impl ExecOutbox for Bridge {
        fn publish_once(&mut self, ev: Event) -> Result<()> {
        if !self.seen.insert(ev.id) {
            return Ok(());
        }
        // ЖБ-контракт:
        // - writer.append MUST be idempotent by EventId
        // - duplicate EventId -> NO-OP
        
        let payload = serde_json::to_vec(&ev)?;
        let kind = "event";
        let ts_ns: u64 = ev.ts_proc.nanos.try_into()?;
        self.writer.append_bytes(kind, ts_ns, &payload)?;
        self.writer.flush()?;

        Ok(())
    }
}
