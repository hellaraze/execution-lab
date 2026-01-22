use risk::{RiskConfig, RiskEngine};

#[test]
fn default_cfg_is_unbounded() {
    let cfg = RiskConfig::default();
    assert!(cfg.max_pos.is_infinite());
    assert!(cfg.max_notional.is_infinite());
}

#[test]
fn engine_exposes_cfg() {
    let cfg = RiskConfig {
        max_pos: 1.0,
        max_notional: 2.0,
    };
    let eng = RiskEngine::new(cfg.clone());
    assert_eq!(eng.cfg().max_pos, cfg.max_pos);
    assert_eq!(eng.cfg().max_notional, cfg.max_notional);
}
