use anyhow::Result;
use eventlog::writer::{Durability, EventLogWriter};

use exec::events::{ExecEvent, OrderId};
use exec::util::instrument::InstrumentKey;

fn main() -> Result<()> {
    let out_path = std::env::args()
        .nth(1)
        .expect("usage: gen_exec_fixture <out.eventlog>");

    let mut w = EventLogWriter::open_append(&out_path, "exec_fixture", Durability::Buffered)?;

    let btc = InstrumentKey::new("binance", "BTCUSDT");
    let eth = InstrumentKey::new("binance", "ETHUSDT");

    // Deterministic, minimal-but-covering lifecycle across two instruments.
    let events = [
        ExecEvent::OrderCreated {
            instrument: btc.clone(),
            id: OrderId(1),
        },
        ExecEvent::OrderAcked {
            instrument: btc.clone(),
            id: OrderId(1),
        },
        ExecEvent::OrderFill {
            instrument: btc.clone(),
            id: OrderId(1),
            filled_qty: 0.5,
            avg_px: 100.0,
        },
        ExecEvent::OrderCancelRequested {
            instrument: btc.clone(),
            id: OrderId(1),
        },
        ExecEvent::OrderCancelled {
            instrument: btc.clone(),
            id: OrderId(1),
        },
        ExecEvent::OrderCreated {
            instrument: eth.clone(),
            id: OrderId(2),
        },
        ExecEvent::OrderRejected {
            instrument: eth.clone(),
            id: OrderId(2),
            reason: "rejected_fixture".to_string(),
        },
    ];

    for ev in events {
        w.write(&ev)?;
    }

    w.flush()?;
    Ok(())
}
