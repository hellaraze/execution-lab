use std::collections::HashMap;
use crate::fsm::OrderState;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub state: OrderState,
}

#[derive(Default)]
pub struct OrderStore {
    orders: HashMap<u64, Order>,
}

impl OrderStore {
    pub fn get_or_create(&mut self, id: u64) -> &mut Order {
        self.orders.entry(id).or_insert(Order {
            id,
            state: OrderState::New,
        })
    }
}
