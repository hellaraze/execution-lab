use clap::Parser;

#[cfg(feature = "replay-ro")]
use anyhow::{bail, Context};

#[cfg(feature = "replay-ro")]
use el_core::event::{Event, EventPayload, EventType};

#[derive(Parser, Debug)]
struct Args {
    /// input eventlog (MD)
    in_path: String,
    /// output eventlog
    out_path: String,
    /// shift all TickerBbo bid/ask by +bps (multiply by k)
    #[arg(long)]
    shift_bps: f64,
}

#[cfg(not(feature = "replay-ro"))]
fn main() -> anyhow::Result<()> {
    anyhow::bail!("run with --features replay-ro")
}

#[cfg(feature = "replay-ro")]
fn main() -> anyhow::Result<()> {
    let a = Args::parse();
    let k = 1.0 + (a.shift_bps / 10_000.0);

    let mut r = eventlog::EventLogReader::open(&a.in_path)?;
    let mut w = eventlog::EventLogWriter::open(&a.out_path)?;

    let mut n_in: u64 = 0;
    let mut n_out: u64 = 0;

    while let Some((_env, payload)) = r.read_next()? {
        n_in += 1;

        let mut e: Event = serde_json::from_slice(&payload)?;

        // emit BBO-only stream:
        // - pass through shifted TickerBbo
        // - OR convert BookSnapshot top-of-book -> TickerBbo, then shift
        match (&e.event_type, &e.payload) {
            (EventType::TickerBbo, EventPayload::TickerBbo { .. }) => {
                // ok, we will shift in-place below
            }
            (EventType::BookSnapshot, EventPayload::BookSnapshot { bids, asks }) => {
                let (bid, _bq) = bids.first().context("BookSnapshot bids empty")?;
                let (ask, _aq) = asks.first().context("BookSnapshot asks empty")?;
                // convert this event into TickerBbo (keep id/instrument/ts)
                e.event_type = EventType::TickerBbo;
                e.payload = EventPayload::TickerBbo { bid: *bid, ask: *ask };
            }
            _ => continue,
        }

        // now must be TickerBbo
        match &mut e.payload {
            EventPayload::TickerBbo { bid, ask } => {
                *bid *= k;
                *ask *= k;
                if !bid.is_finite() || !ask.is_finite() || *bid <= 0.0 || *ask <= 0.0 || *bid > *ask {
                    bail!("invalid shifted bbo: bid={} ask={} (k={})", bid, ask, k);
                }
            }
            _ => continue,
        }

        // write as normal eventlog entry
        w.write(&e)?;
        n_out += 1;
    }

    w.flush()?;

    println!("wrote bbo-only synth: in={} out={} events_in={} events_out={} k={}", a.in_path, a.out_path, n_in, n_out, k);
    Ok(())
}
