use audit::{chain_step, hash_bytes, Hash32};

#[test]
fn hash_bytes_is_deterministic() {
    let a = hash_bytes(b"hello");
    let b = hash_bytes(b"hello");
    assert_eq!(a, b);
}

#[test]
fn chain_changes_with_payload() {
    let h0 = Hash32::ZERO;
    let h1 = chain_step(h0, b"a");
    let h2 = chain_step(h0, b"b");
    assert_ne!(h1, h2);
}

#[test]
fn chain_is_order_sensitive() {
    let h0 = Hash32::ZERO;
    let ha = chain_step(h0, b"a");
    let hab = chain_step(ha, b"b");

    let hb = chain_step(h0, b"b");
    let hba = chain_step(hb, b"a");

    assert_ne!(hab, hba);
}
