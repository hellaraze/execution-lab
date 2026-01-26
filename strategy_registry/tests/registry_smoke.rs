use strategy_registry::StrategyRegistry;
use strategy_sdk::example::AlwaysNoGas;

#[test]
fn registry_register_and_get() {
    let mut reg = StrategyRegistry::new();
    reg.register(Box::new(AlwaysNoGas)).unwrap();

    let s = reg.get("always_no_gas").unwrap();
    assert_eq!(s.name(), "always_no_gas");
}
