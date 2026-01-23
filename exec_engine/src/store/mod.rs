use std::collections::{HashMap, HashSet};

use crate::error::ExecError;
use crate::fsm::{OrderData, OrderEvent};

#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub data: OrderData,
    fill_qty: HashMap<u64, u64>,
    seen_fills: HashSet<u64>,
}

pub struct OrderStore {
    orders: HashMap<u64, Order>,
}

impl OrderStore {
    pub fn new() -> Self {
        Self { orders: HashMap::new() }
    }

    pub fn get_or_create(&mut self, id: u64, total_atoms: u64) -> Result<&mut Order, ExecError> {
        use std::collections::hash_map::Entry;

        match self.orders.entry(id) {
            Entry::Occupied(mut o) => {
                if o.get().data.total_atoms != total_atoms {
                    return Err(ExecError::ConfigMismatch);
                }
                Ok(o.into_mut())
            }
            Entry::Vacant(v) => Ok(v.insert(Order {
                id,
                data: OrderData::new(total_atoms),
                fill_qty: HashMap::new(),
                seen_fills: HashSet::new(),
            })),
        }
    }

    pub fn apply(&mut self, id: u64, ev: OrderEvent) -> Result<(), ExecError> {
        let order = self.orders.get_mut(&id).ok_or(ExecError::NotFound)?;

        if let OrderEvent::Fill { fill_id, qty_atoms } = &ev {
            if let Some(prev) = order.fill_qty.get(fill_id) {
                if prev != qty_atoms {
                    return Err(ExecError::InvalidTransition);
                }
                return Ok(());
            }
            order.fill_qty.insert(*fill_id, *qty_atoms);
            order.seen_fills.insert(*fill_id);
        }

        crate::fsm::apply(&mut order.data, &ev)
    }
}
