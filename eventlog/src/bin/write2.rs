use anyhow::Result;
use eventlog::writer::{Durability, EventLogWriter};
use serde_json::json;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/el_test.log".to_string());

    // open #1
    {
        let mut w = EventLogWriter::open_append(&path, "el:test", Durability::Buffered)?;
        let seq1 = w.append_json_value("event", 1, &json!({"hello": 1}))?;
        w.flush()?;
        println!("seq1={}", seq1);
    }

    // open #2 (must continue)
    {
        let mut w = EventLogWriter::open_append(&path, "el:test", Durability::Buffered)?;
        let seq2 = w.append_json_value("event", 2, &json!({"hello": 2}))?;
        w.flush()?;
        println!("seq2={}", seq2);
    }

    Ok(())
}
