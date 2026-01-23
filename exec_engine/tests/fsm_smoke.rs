use exec_engine::fsm::{apply, OrderEvent, OrderState};

#[test]
fn basic_flow() {
    let mut s = OrderState::New;

    s = apply(s, &OrderEvent::Accept);
    assert_eq!(s, OrderState::Open);

    s = apply(s, &OrderEvent::Fill { qty: 1.0 });
    assert!(matches!(s, OrderState::PartiallyFilled | OrderState::Filled));

    s = apply(s, &OrderEvent::Cancel);
    assert_eq!(s, OrderState::Canceled);
}
