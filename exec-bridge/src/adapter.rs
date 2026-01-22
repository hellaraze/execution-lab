use el_core::event::Event;
use el_exec::event::ExecEvent;

pub fn adapt(event: Event) -> Option<ExecEvent> {
    match event {
        Event::Exec(e) => Some(e),
        _ => None,
    }
}
