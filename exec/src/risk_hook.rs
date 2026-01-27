use el_obs::event::ObsEvent;
use el_obs::sink::ObsSink;
use el_risk::contract::{RiskEngine, RiskInput, RiskVerdict};

pub fn risk_precheck<E: RiskEngine, S: ObsSink>(
    engine: &E,
    sink: &mut S,
    input: &RiskInput,
) -> RiskVerdict {
    let verdict = engine.evaluate(input);
    sink.emit(ObsEvent::RiskEvaluated {
        ts: input.ts,
        verdict: format!("{:?}", verdict),
    });
    verdict
}
