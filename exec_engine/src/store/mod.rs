use std::collections::{HashMap, HashSet};

use crate::fsm::{OrderData, OrderEvent};
use crate::error::ExecError;

#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub data: OrderData,
    seen_fills: HashSet<u64>,
}

pub struct OrderStore {
    orders: HashMap<u64, Order>,
}

impl OrderStore {
    pub fn new() -> Self {
        Self { orders: HashMap::new() }
    }

    pub fn get_or_create(&mut self, id: u64, total_atoms: u64) -> &mut Order {
        self.orders.entry(id).or_insert(Order {
            id,
            data: OrderData::new(total_atoms),
            seen_fills: HashSet::new(),
        })
    }

    pub fn apply(&mut self, id: u64, ev: OrderEvent) -> Result<(), ExecError> {
        let order = self.orders.get_mut(&id).ok_or(ExecError::InvalidTransition)?;

        if let OrderEvent::Fill { fill_id, .. } = &ev {
            if !order.seen_fills.insert(*fill_id) {
                // idempotent replay
                return Ok(());
            }
        }

        crate::fsm::apply(&mut order.data, &ev)
    }
}
