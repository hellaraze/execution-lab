Execution Bridge — Phase 4 Invariants

1. EXACTLY-ONCE
- Each core::EventId is appended to eventlog at most once
- Duplicate EventId => NO-OP

2. RESTART-SAFE
- On restart, bridge MUST replay existing log and seed dedup set
- No duplicate events after crash/restart

3. ORDER PRESERVATION
- Append order equals publish order for unique EventId

4. REPLAY COMPATIBLE
- eventlog produced by bridge MUST replay deterministically

5. FLUSH CONTRACT
- publish_once() flushes before returning success

6. SCOPE
- Bridge works ONLY on el_core::event::Event
- ExecEvent is out of scope for Phase 4
