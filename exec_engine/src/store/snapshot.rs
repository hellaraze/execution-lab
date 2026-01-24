use serde::{Deserialize, Serialize};

use crate::fsm::OrderState;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderSnapshot {
    pub id: u64,
    pub state: OrderState,
    pub total_atoms: u64,
    pub filled_atoms: u64,
    // sorted by fill_id asc, stable JSON
    pub fill_qty: Vec<(u64, u64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotV1 {
    pub version: u32,
    pub order: OrderSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum SnapshotEnvelope {
    V1(SnapshotV1),
}

impl SnapshotEnvelope {
    pub fn v1(order: OrderSnapshot) -> Self {
        SnapshotEnvelope::V1(SnapshotV1 { version: 1, order })
    }
}
