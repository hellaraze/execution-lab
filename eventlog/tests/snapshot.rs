use eventlog::snapshot::Snapshot;

#[test]
#[should_panic(expected = "SNAPSHOT HASH MISMATCH")]
fn snapshot_hash_mismatch_panics() {
    let s1 = Snapshot::new(1u64);
    let s2 = Snapshot::new(2u64);

    s1.assert_same(&s2);
}
