use std::path::PathBuf;

use eventlog::EventLogWriter;
use eventlog::writer::Durability;

use exec::events::{ExecEvent, OrderId};
use exec::io::read_exec_events_from_eventlog;
use exec::order::build_snapshot;

fn tmp_path(name: &str) -> PathBuf {
    let pid = std::process::id();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("execution-lab-{}-{}-{}.log", name, pid, now))
}

#[test]
fn eventlog_exec_events_roundtrip_snapshot_hash() {
    let path = tmp_path("exec");

    // Write a small exec event stream into eventlog
    let mut w = EventLogWriter::open_append(&path, "exec:test", Durability::Buffered).unwrap();

    let id = OrderId(42);
    let events = vec![
        ExecEvent::OrderCreated { id },
        ExecEvent::OrderValidated { id },
        ExecEvent::OrderSent { id },
        ExecEvent::OrderAcked { id },
        ExecEvent::OrderFill { id, filled_qty: 1.0, avg_px: 100.0 },
        ExecEvent::OrderCancelRequested { id },
        ExecEvent::OrderCancelled { id },
    ];

    for ev in &events {
        w.write(ev).unwrap();
    }
    w.flush().unwrap();

    // Read back
    let got = read_exec_events_from_eventlog(&path).unwrap();
    assert_eq!(got, events);

    // Snapshot hash (golden)
    let (_store, h) = build_snapshot(&got).unwrap();

    // First run: lock this value.
    assert_eq!(h, 5363716124260894211u64);

    // cleanup best-effort
    let _ = std::fs::remove_file(&path);
}
