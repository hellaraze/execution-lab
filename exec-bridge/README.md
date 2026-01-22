# exec-bridge

Maps `el_core::event::Event` into `exec::events::ExecEvent`.

## Notes / TODO
- Today we map instrument into `exec::util::instrument::InstrumentKey` via `format!("{:?}", exchange)` + `symbol`.
- Next: unify InstrumentKey type between core and exec (single source of truth) to avoid lossy mapping.
- OrderId mapping currently parses `order_id: String` into `OrderId(u64)`; decide canonical order id format across layers.
