use std::collections::HashMap;

use anyhow::Result;

use crate::events::{ExecEvent, OrderId};
use super::types::{OrderState, OrderView};

#[derive(Debug, Default)]
pub struct OrderStore {
    by_id: HashMap<OrderId, OrderView>,
}

impl OrderStore {
    pub fn new() -> Self {
        Self { by_id: HashMap::new() }
    }

    pub fn apply_all(&mut self, events: &[ExecEvent]) -> Result<()> {
        for ev in events {
            self.apply(ev)?;
        }
        Ok(())
    }

    pub fn apply(&mut self, ev: &ExecEvent) -> Result<()> {
        match ev {
            ExecEvent::OrderCreated { id } => {
                self.by_id.insert(*id, OrderView::new());
            }
            ExecEvent::OrderValidated { id } => {
                let v = self.by_id.entry(*id).or_insert_with(OrderView::new);
                v.state = OrderState::Validated;
            }
            ExecEvent::OrderSent { id } => {
                let v = self.by_id.entry(*id).or_insert_with(OrderView::new);
                v.state = OrderState::Sent;
            }
            ExecEvent::OrderAcked { id } => {
                let v = self.by_id.entry(*id).or_insert_with(OrderView::new);
                v.state = OrderState::Acknowledged;
            }
            ExecEvent::OrderFill { id, filled_qty, avg_px } => {
                if !filled_qty.is_finite() || !avg_px.is_finite() || *filled_qty < 0.0 || *avg_px < 0.0 {
                    anyhow::bail!("invalid fill numbers: filled_qty={} avg_px={}", filled_qty, avg_px);
                }
                let v = self.by_id.entry(*id).or_insert_with(OrderView::new);
                v.filled_qty = *filled_qty;
                v.avg_px = *avg_px;

                // state update
                if *filled_qty == 0.0 {
                    // no-op
                } else if v.state == OrderState::Acknowledged || v.state == OrderState::PartiallyFilled {
                    v.state = OrderState::PartiallyFilled;
                }
            }
            ExecEvent::OrderCancelRequested { id } => {
                let v = self.by_id.entry(*id).or_insert_with(OrderView::new);
                v.state = OrderState::CancelRequested;
            }
            ExecEvent::OrderCancelled { id } => {
                let v = self.by_id.entry(*id).or_insert_with(OrderView::new);
                v.state = OrderState::Cancelled;
            }
            ExecEvent::OrderRejected { id, .. } => {
                let v = self.by_id.entry(*id).or_insert_with(OrderView::new);
                v.state = OrderState::Rejected;
            }
            ExecEvent::OrderExpired { id } => {
                let v = self.by_id.entry(*id).or_insert_with(OrderView::new);
                v.state = OrderState::Expired;
            }
        }
        Ok(())
    }

    pub fn view(&self, id: OrderId) -> Option<&OrderView> {
        self.by_id.get(&id)
    }
}
