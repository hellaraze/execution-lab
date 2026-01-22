use crate::adapter::{CancelOrder, ExecAdapter, ExecResult, PlaceOrder};

pub struct MockAdapter;

impl ExecAdapter for MockAdapter {
    fn place_order(&mut self, _cmd: PlaceOrder) -> ExecResult {
        ExecResult::Accepted
    }

    fn cancel_order(&mut self, _cmd: CancelOrder) -> ExecResult {
        ExecResult::Accepted
    }
}
