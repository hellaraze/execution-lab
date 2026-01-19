use anyhow::Result;
use el_core::event::{Event, EventPayload, EventType, Exchange};
use el_core::time::{Timestamp, TimeSource};
use eventlog::EventLogWriter;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

fn make_delta(p: f64, q: f64) -> Event {
    Event {
        id: uuid::Uuid::new_v4(),
        event_type: EventType::BookDelta,
        exchange: Exchange::Binance,
        symbol: "BTCUSDT".to_string(),
        ts_exchange: None,
        ts_recv: Timestamp::new(0, TimeSource::Receive),
        ts_proc: Timestamp::new(0, TimeSource::Process),
        seq: None,
        schema_version: 1,
        integrity_flags: vec![],
        payload: EventPayload::BookDelta {
            bids: vec![(p, q)],
            asks: vec![],
        },
        meta: Default::default(),
    }
}

fn write_log(path: &str, deltas: &[Event]) -> Result<()> {
    let mut w = EventLogWriter::open(path)?;
    for ev in deltas {
        w.write(ev)?;
    }
    w.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    std::fs::create_dir_all("replay/tests/data")?;

    let mut deltas: Vec<Event> = vec![];
    for i in 0..40 {
        // enough steps to be meaningful
        deltas.push(make_delta(50_000.0 + i as f64, 1.0 + (i % 7) as f64 * 0.1));
    }

    // A: original order
    write_log("replay/tests/data/comm_A.log", &deltas)?;

    // B: shuffled order (same elements)
    let mut shuffled = deltas.clone();
    let mut rng = StdRng::seed_from_u64(42);
    shuffled.shuffle(&mut rng);
    write_log("replay/tests/data/comm_B.log", &shuffled)?;

    println!("generated comm_A.log and comm_B.log");
    Ok(())
}
