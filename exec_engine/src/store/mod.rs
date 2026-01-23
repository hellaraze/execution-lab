use std::collections::{HashMap, HashSet};

use crate::error::ExecError;
use crate::fsm::{OrderData, OrderEvent};

#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub data: OrderData,
    seen_fills: HashSet<u64>,
    fill_qty: HashMap<u64, u64>,
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
            fill_qty: HashMap::new(),
        })
    }

    pub fn apply(&mut self, id: u64, ev: OrderEvent) -> Result<(), ExecError> {
        let order = self.orders.get_mut(&id).ok_or(ExecError::InvalidTransition)?;

        if let OrderEvent::Fill { fill_id, qty_atoms } = &ev {
            if let Some(prev) = order.fill_qty.get(fill_id) {
                if prev != qty_atoms {
                    return Err(ExecError::InvalidTransition);
                }
                // identical replay
                return Ok(());
            }
            order.fill_qty.insert(*fill_id, *qty_atoms);

            // also keep a set guard (belt + suspenders)
            order.seen_fills.insert(*fill_id);
        }

        crate::fsm::apply(&mut order.data, &ev)
    }
}
