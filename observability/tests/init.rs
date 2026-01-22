use observability::{init_prometheus, ObsError};

#[test]
fn init_is_one_shot() {
    let h = init_prometheus().expect("first init must succeed");
    let _txt = h.render();

    let res = init_prometheus();
    assert!(matches!(res, Err(ObsError::AlreadyInstalled)));
}
