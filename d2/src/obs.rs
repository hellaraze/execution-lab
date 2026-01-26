use el_obs::event::ObsEvent;
use el_obs::sink::{FileSink, NoopSink, ObsSink};
use el_core::time::{Timestamp, TimeSource};

pub enum Obs {
    File(FileSink),
    Noop(NoopSink),
}

impl Obs {
    pub fn open(opt_path: Option<&str>) -> Self {
        match opt_path {
            Some(p) => Self::File(FileSink::open_append(p).expect("open obs file")),
            None => Self::Noop(NoopSink),
        }
    }
}

impl ObsSink for Obs {
    fn emit(&mut self, ev: ObsEvent) {
        match self {
            Obs::File(s) => s.emit(ev),
            Obs::Noop(s) => s.emit(ev),
        }
    }
}

pub fn emit_decision_at(sink: &mut impl ObsSink, ts: Timestamp, verdict: &str) {
    sink.emit(ObsEvent::RiskEvaluated {
        ts,
        verdict: verdict.to_string(),
    });
}

#[cfg(feature = "replay-ro")]
pub fn ts_from_event(e: &el_core::event::Event) -> Timestamp {
    // In our schema, ts_recv/ts_proc are Timestamps (not Option).
    // Prefer proc as "closest to decision", else recv.
    let ts_proc = e.ts_proc;
    let ts_recv = e.ts_recv;

    if ts_proc.nanos != 0 {
        return ts_proc;
    }
    if ts_recv.nanos != 0 {
        return ts_recv;
    }
    Timestamp::new(0, TimeSource::Process)
}
