use anyhow::Result;

mod golden; // reuse helper from tests/golden.rs

#[test]
fn replay_is_order_sensitive() -> Result<()> {
    let a = golden::run_and_collect_chain_hashes("tests/data/comm_A.log", 100)?;
    let b = golden::run_and_collect_chain_hashes("tests/data/comm_B.log", 100)?;

    anyhow::ensure!(
        a != b,
        "CRITICAL: replay is commutative — different order produced same chain"
    );

    Ok(())
}
