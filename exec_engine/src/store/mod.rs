use std::collections::{HashMap, HashSet};

use crate::error::ExecError;
use crate::fsm::{OrderData, OrderEvent};

#[derive(Debug)]
pub mod snapshot;

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
            Entry::Occupied(o) => {
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

    pub fn export_snapshot(&self, id: u64) -> Result<crate::store::snapshot::OrderSnapshot, ExecError> {
        let o = self.orders.get(&id).ok_or(ExecError::NotFound)?;
        Ok(crate::store::snapshot::OrderSnapshot {
            id: o.id,
            state: o.data.state,
            total_atoms: o.data.total_atoms,
            filled_atoms: o.data.filled_atoms,
            fill_qty: o.fill_qty.clone(),
        })
    }

    pub fn import_snapshot(&mut self, snap: crate::store::snapshot::OrderSnapshot) {
        let mut seen = HashSet::new();
        for k in snap.fill_qty.keys() { seen.insert(*k); }
        self.orders.insert(snap.id, Order {
            id: snap.id,
            data: crate::fsm::OrderData { state: snap.state, total_atoms: snap.total_atoms, filled_atoms: snap.filled_atoms },
            fill_qty: snap.fill_qty,
            seen_fills: seen,
        });
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
