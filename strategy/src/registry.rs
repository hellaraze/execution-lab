use strategy_registry::StrategyRegistry;
use strategy_sdk::example::AlwaysNoGas;
use strategy_sdk::d2_strategy::D2SignalStrategy;

pub fn build_default_registry() -> StrategyRegistry {
    let mut r = StrategyRegistry::new();
    r.register(Box::new(AlwaysNoGas)).expect("register AlwaysNoGas");
    r.register(Box::new(D2SignalStrategy)).expect("register D2SignalStrategy");
    r
}
