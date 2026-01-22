use observability::{handle, init_prometheus, ObsError};

#[test]
fn init_is_one_shot() {
    assert!(handle().is_none());

    let h = init_prometheus().expect("first init must succeed");
    let _txt = h.render();
    assert!(handle().is_some());

    let res = init_prometheus();
    assert!(matches!(res, Err(ObsError::AlreadyInstalled)));
    assert!(handle().is_some());
}
