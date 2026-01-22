use anyhow::{Context, Result};
use std::path::Path;

use eventlog::EventLogReader;

use crate::events::ExecEvent;

/// Read a log file written by `eventlog`, parse payload bytes as JSON `ExecEvent`.
/// This is strict: if a line payload can't be parsed as ExecEvent -> error.
pub fn read_exec_events_from_eventlog(path: impl AsRef<Path>) -> Result<Vec<ExecEvent>> {
    let path = path.as_ref();

    let mut r = EventLogReader::open(path).with_context(|| format!("open eventlog {:?}", path))?;
    let mut out = Vec::new();

    while let Some((env, payload)) = r.next().context("read next eventlog line")? {
        // payload bytes are JSON of the event (writer.write<T> uses serde_json::to_vec)
        let ev: ExecEvent = serde_json::from_slice(&payload).with_context(|| {
            format!(
                "parse ExecEvent: seq={} stream={} kind={}",
                env.seq, env.stream, env.kind
            )
        })?;
        out.push(ev);
    }

    Ok(out)
}
