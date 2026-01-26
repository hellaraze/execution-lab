#[cfg(feature = "replay-ro")]
#[test]
fn replay_ro_typechecks() {
    fn _typecheck(_e: &el_core::event::Event) {}
}
