use el_obs::contracts_shadow::ContractShadowWriter;

use el_contracts::v1;
use el_core::event::Exchange;
use el_core::instrument::InstrumentKey;
use el_core::time::{TimeSource, Timestamp};

#[test]
fn writes_jsonl() {
    let tmp = std::env::temp_dir().join("el_obs_contract_shadow_test.jsonl");
    let mut w = ContractShadowWriter::open(&tmp).unwrap();

    let instrument = InstrumentKey::new(Exchange::Binance, "BTCUSDT");
    let ts = Timestamp::new(123, TimeSource::Process);

    let md = v1::md::MdEvent {
        instrument: instrument.clone(),
        ts,
        bbo: v1::md::Bbo {
            bid_px: 1.0,
            bid_qty: 2.0,
            ask_px: 3.0,
            ask_qty: 4.0,
        },
    };

    let sd = v1::strategy::StrategyDecision {
        instrument: instrument.clone(),
        ts,
        decision: v1::strategy::Decision::NoGas,
        edge_bps: -12.34,
    };

    let au = v1::audit::AuditEvent {
        ts,
        source: "test",
        message: "ok",
    };

    w.write_md(&md).unwrap();
    w.write_strategy(&sd).unwrap();
    w.write_audit(&au).unwrap();
    w.flush().unwrap();

    let s = std::fs::read_to_string(&tmp).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines.len(), 3);
    assert!(lines[0].contains("\"kind\":\"md\""));
    assert!(lines[1].contains("\"kind\":\"strategy\""));
    assert!(lines[2].contains("\"kind\":\"audit\""));
}
