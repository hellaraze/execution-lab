use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use el_core::event::Exchange;
use el_core::time::{TimeSource, Timestamp};
use el_obs::sink::NoopSink;
use el_risk::decision::SimpleRisk;
use el_risk::limits::RiskLimits;
use eventlog::EventLogWriter;
use exec::adapter::{CancelOrder, ExecAdapter, ExecResult, PlaceOrder};
use exec::risk_hook::risk_precheck;

#[derive(Debug, Clone, Copy)]
pub enum LiveMode {
    DryRun,
    PostOnly,
    Live,
}

#[derive(Clone)]
pub struct KillSwitch(Arc<AtomicBool>);

impl KillSwitch {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }
    pub fn is_killed(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
    pub fn kill(&self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

pub struct BinanceLiveAdapter {
    mode: LiveMode,
    risk: SimpleRisk,
    kill: KillSwitch,
    sink: NoopSink,
    w: Option<EventLogWriter>,
}

impl BinanceLiveAdapter {
    pub fn new(mode: LiveMode, limits: RiskLimits) -> Self {
        Self {
            mode,
            risk: SimpleRisk { limits },
            kill: KillSwitch::new(),
            sink: NoopSink,
            w: None,
        }
    }

    pub fn with_eventlog(mut self, path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        std::fs::create_dir_all(path.as_ref().parent().unwrap_or(std::path::Path::new(".")))?;
        self.w = Some(EventLogWriter::open(path.as_ref())?);
        Ok(self)
    }

    pub fn kill_switch(&self) -> KillSwitch {
        self.kill.clone()
    }

    fn log_json(&mut self, kind: &str, v: &serde_json::Value) {
        if let Some(w) = self.w.as_mut() {
            let _ = w.append_json_value(kind, 0, v);
        }
    }

    fn precheck(&mut self, cmd: &PlaceOrder) -> Result<(), String> {
        if self.kill.is_killed() {
            return Err("KILL_SWITCH".into());
        }

        // Risk precheck must be BEFORE any send.
        let notional = cmd.price * cmd.qty;
        let input = el_risk::contract::RiskInput {
            ts: Timestamp::new(0, TimeSource::Exchange),
            instrument: cmd.instrument.clone(),
            notional,
            side: match cmd.side {
                exec::adapter::Side::Buy => el_risk::contract::Side::Buy,
                exec::adapter::Side::Sell => el_risk::contract::Side::Sell,
            },
        };

        match risk_precheck(&self.risk, &mut self.sink, &input) {
            el_risk::contract::RiskVerdict::Allow => Ok(()),
            el_risk::contract::RiskVerdict::Block(reason) => {
                Err(format!("RISK_BLOCK: {:?}", reason))
            }
        }
    }
}

impl ExecAdapter for BinanceLiveAdapter {
    fn place_order(&mut self, cmd: PlaceOrder) -> ExecResult {
        self.log_json(
            "exec_cmd_place",
            &serde_json::json!({
                "exchange": "binance",
                "mode": format!("{:?}", self.mode),
                "instrument": cmd.instrument.symbol,
                "order_id": cmd.order_id.0,
                "price": cmd.price,
                "qty": cmd.qty,
            }),
        );

        if let Err(r) = self.precheck(&cmd) {
            self.log_json("exec_reject", &serde_json::json!({"reason": r}));
            return ExecResult::Rejected { reason: r };
        }

        match self.mode {
            LiveMode::DryRun => {
                self.log_json("exec_accept", &serde_json::json!({"note":"dry_run_accept"}));
                ExecResult::Accepted
            }
            LiveMode::PostOnly | LiveMode::Live => {
                // Phase 5 minimal: real REST/WebSocket wiring comes next.
                // For now: we refuse rather than pretend we traded.
                let r = "NOT_IMPLEMENTED_BINANCE_WIRE".to_string();
                self.log_json("exec_reject", &serde_json::json!({"reason": r}));
                ExecResult::Rejected { reason: r }
            }
        }
    }

    fn cancel_order(&mut self, cmd: CancelOrder) -> ExecResult {
        self.log_json(
            "exec_cmd_cancel",
            &serde_json::json!({
                "exchange": "binance",
                "order_id": cmd.order_id.0,
            }),
        );

        if self.kill.is_killed() {
            return ExecResult::Rejected {
                reason: "KILL_SWITCH".into(),
            };
        }

        // Minimal: refuse until wire exists.
        ExecResult::Rejected {
            reason: "NOT_IMPLEMENTED_BINANCE_WIRE".into(),
        }
    }
}

// smoke helper for callers that want a typed Exchange
pub const BINANCE: Exchange = Exchange::Binance;
