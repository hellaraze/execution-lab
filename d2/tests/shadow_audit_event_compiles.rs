use d2::contracts_shadow::shadow_audit_event;
use el_core::time::{TimeSource, Timestamp};

#[test]
fn shadow_audit_event_builds() {
    let ts = Timestamp::new(777, TimeSource::Process);
    let ev = shadow_audit_event(ts, "d2", "hello");

    assert_eq!(ev.ts, ts);
    assert_eq!(ev.source, "d2");
    assert_eq!(ev.message, "hello");
}
