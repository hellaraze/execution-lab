use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::fsm::OrderState;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderSnapshot {
    pub id: u64,
    pub state: OrderState,
    pub total_atoms: u64,
    pub filled_atoms: u64,
    pub fill_qty: HashMap<u64, u64>,
}
