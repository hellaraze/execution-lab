use std::collections::HashMap;

use crate::events::{ExecEvent, OrderId};
use crate::order::{apply, FsmError, OrderView};

#[derive(Debug, Default)]
pub struct OrderStore {
    views: HashMap<OrderId, OrderView>,
}

impl OrderStore {
    pub fn new() -> Self {
        Self { views: HashMap::new() }
    }

    pub fn view(&self, id: OrderId) -> Option<&OrderView> {
        self.views.get(&id)
    }

    pub fn upsert_default(&mut self, id: OrderId) -> &mut OrderView {
        self.views.entry(id).or_insert_with(OrderView::new)
    }

    pub fn apply_event(&mut self, ev: &ExecEvent) -> Result<(), FsmError> {
        let id = match ev {
            ExecEvent::OrderCreated { id }
            | ExecEvent::OrderValidated { id }
            | ExecEvent::OrderSent { id }
            | ExecEvent::OrderAcked { id }
            | ExecEvent::OrderFill { id, .. }
            | ExecEvent::OrderCancelRequested { id }
            | ExecEvent::OrderCancelled { id }
            | ExecEvent::OrderRejected { id, .. }
            | ExecEvent::OrderExpired { id } => *id,
        };

        let view = self.upsert_default(id);
        apply(view, id, ev)
    }

    pub fn apply_all<'a, I>(&mut self, events: I) -> Result<(), FsmError>
    where
        I: IntoIterator<Item = &'a ExecEvent>,
    {
        for ev in events {
            self.apply_event(ev)?;
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.views.len()
    }

    pub fn is_empty(&self) -> bool {
        self.views.is_empty()
    }
}
