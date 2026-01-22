use execution_bridge::*;
use eventlog::mem::MemEventLog;

#[test]
fn exactly_once_append_is_idempotent() {
    let mut log = MemEventLog::default();
    let mut bridge = Bridge::new(&mut log);

    // временно: пока не знаем точный конструктор ExecEvent в твоём core.
    // ниже будет компил-ошибка -> я дам точные правки по твоим типам.
    // IMPORTANT: НЕ КОВЫРЯЙ, просто запускай test и скинь ошибку.
    let ev = todo!("provide real ExecEvent constructor from your el_core");

    bridge.publish_once(ev.clone()).unwrap();
    bridge.publish_once(ev.clone()).unwrap();

    let events = log.read_all();
    assert_eq!(events.len(), 1);
}
