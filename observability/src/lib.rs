use metrics::counter;

pub fn record_ingest_ok() {
    counter!("ingest_ok_total");
}

// TODO(phase5): wire a recorder + real gauges.
// For now keep compile-clean API.
pub fn set_book_depth(_depth: f64) {
    // noop
}
