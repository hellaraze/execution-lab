use indexmap::IndexMap;
use strategy_sdk::Strategy;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("strategy already registered: {0}")]
    Duplicate(&'static str),
    #[error("strategy not found: {0}")]
    NotFound(&'static str),
}

pub struct StrategyRegistry {
    map: IndexMap<&'static str, Box<dyn Strategy>>,
}

impl StrategyRegistry {
    pub fn new() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }

    pub fn register(&mut self, s: Box<dyn Strategy>) -> Result<(), RegistryError> {
        let name = s.name();
        if self.map.contains_key(name) {
            return Err(RegistryError::Duplicate(name));
        }
        self.map.insert(name, s);
        Ok(())
    }

    pub fn get(&self, name: &'static str) -> Result<&dyn Strategy, RegistryError> {
        self.map
            .get(name)
            .map(|b| b.as_ref())
            .ok_or(RegistryError::NotFound(name))
    }

    pub fn list(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.map.keys().copied()
    }
}

impl Default for StrategyRegistry {
    fn default() -> Self {
        Self::new()
    }
}
