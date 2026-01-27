use std::fs::File;
use std::io::{BufWriter, Write};

use el_core::event::Event;
use el_core::instrument::InstrumentKey;
use el_core::time::{TimeSource, Timestamp};

use el_obs::contracts_shadow::ContractShadowWriter;

use crate::signal::{DecisionReason, GasDecision};

pub struct Obs {
    out_path: Option<String>,
    out: Option<BufWriter<File>>,
    contracts: Option<ContractShadowWriter>,
}

impl Obs {
    pub fn open(path: Option<&str>) -> Self {
        let out_path = path.map(|s| s.to_string());

        // primary obs writer (legacy): create file if path is set and not "-"
        let out = path
            .filter(|p| *p != "-")
            .and_then(|p| File::create(p).ok())
            .map(BufWriter::new);

        // contracts side-band
        let mut contracts: Option<ContractShadowWriter> = None;
        if let Some(p) = path.filter(|p| *p != "-") {
            if let Ok(mut w) = ContractShadowWriter::open(format!("{p}.contracts.jsonl")) {
                let ts = Timestamp::new(0, TimeSource::Process);
                let ev = crate::contracts_shadow::shadow_audit_event(ts, "d2", "run_start");
                let _ = w.write_audit(&ev);
                let _ = w.flush();
                contracts = Some(w);
            }
        }

        Self {
            out_path,
            out,
            contracts,
        }
    }

    pub fn path(&self) -> Option<&str> {
        self.out_path.as_deref()
    }

    fn write_legacy_risk_evaluated(&mut self, ts: Timestamp, verdict: &str) {
        if let Some(w) = self.out.as_mut() {
            // legacy format expected by golden:
            // {"RiskEvaluated":{"ts":{...},"verdict":"...decision=Gas..."}}
            let line = serde_json::json!({
                "RiskEvaluated": {
                    "ts": ts,
                    "verdict": verdict
                }
            });
            let _ = serde_json::to_writer(&mut *w, &line);
            let _ = w.write_all(b"\n");
            let _ = w.flush();
        }
    }
}

pub fn ts_from_event(e: &Event) -> Timestamp {
    e.ts_exchange.unwrap_or(e.ts_proc)
}

pub fn emit_decision_at(
    obs: &mut Obs,
    instrument: InstrumentKey,
    ts: Timestamp,
    decision: GasDecision,
    edge_bps: f64,
    reason: DecisionReason,
) {
    // legacy verdict string (golden only needs decision=Gas substring)
    let verdict = format!(
        "decision={:?} reason={:?} edge_bps={:.4}",
        decision, reason, edge_bps
    );

    obs.write_legacy_risk_evaluated(ts, &verdict);

    // contract-shadow side-band
    if let Some(w) = obs.contracts.as_mut() {
        use contracts_bridge::v1 as c1;

        let d = match decision {
            GasDecision::Gas => c1::strategy::Decision::Gas,
            GasDecision::NoGas => c1::strategy::Decision::NoGas,
        };

        let reason_str: &'static str = match reason {
            DecisionReason::Pass => "Pass",
            DecisionReason::BelowEpsilon => "BelowEpsilon",
            DecisionReason::BelowMinEdgeBps => "BelowMinEdgeBps",
            DecisionReason::FeesEatSpread => "FeesEatSpread",
            DecisionReason::InvalidInput => "InvalidInput",
        };

        let sd =
            crate::contracts_shadow::shadow_strategy_decision(instrument.clone(), ts, d, edge_bps);
        let au = crate::contracts_shadow::shadow_audit_event(ts, "d2", reason_str);

        let _ = w.write_strategy(&sd);
        let _ = w.write_audit(&au);
        let _ = w.flush();
    }
}
