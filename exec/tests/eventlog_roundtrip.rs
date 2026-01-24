use anyhow::Result;

use exec::events::{ExecEvent, OrderId};
use exec::util::instrument::InstrumentKey;

use eventlog::{EventLogReader, EventLogWriter};

#[test]
fn eventlog_roundtrip_exec_events() -> Result<()> {
    std::fs::create_dir_all("var")?;
    let path = "var/exec_eventlog_roundtrip.log";
    let _ = std::fs::remove_file(path);

    let instrument = InstrumentKey::new("binance", "BTCUSDT");

    let id = OrderId(123);

    let events = vec![
        ExecEvent::OrderCreated {
            instrument: instrument.clone(),
            id,
        },
        ExecEvent::OrderValidated {
            instrument: instrument.clone(),
            id,
        },
        ExecEvent::OrderSent {
            instrument: instrument.clone(),
            id,
        },
        ExecEvent::OrderAcked {
            instrument: instrument.clone(),
            id,
        },
        ExecEvent::OrderFill {
            instrument: instrument.clone(),
            id,
            filled_qty: 1.0,
            avg_px: 100.0,
        },
        ExecEvent::OrderCancelRequested {
            instrument: instrument.clone(),
            id,
        },
        ExecEvent::OrderCancelled {
            instrument: instrument.clone(),
            id,
        },
    ];

    // write
    let mut w = EventLogWriter::open(path)?;
    for ev in &events {
        let bytes = serde_json::to_vec(ev)?;
        w.append_bytes("event", 0, &bytes)?;
    }
    w.flush()?;

    // read
    let mut r = EventLogReader::open(path)?;
    let mut got: Vec<ExecEvent> = Vec::new();

    while let Some((_env, payload)) = r.read_next()? {
        let ev: ExecEvent = serde_json::from_slice(&payload)?;
        got.push(ev);
    }

    assert_eq!(events, got);
    Ok(())
}
