use std::sync::OnceLock;

use metrics::counter;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

static HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

#[derive(Debug, thiserror::Error)]
pub enum ObsError {
    #[error("metrics recorder already installed")]
    AlreadyInstalled,
}

/// Install Prometheus recorder exactly-once and return a handle.
/// Ultra-JB contract: second call returns Err(AlreadyInstalled).
pub fn init_prometheus() -> Result<&'static PrometheusHandle, ObsError> {
    if HANDLE.get().is_some() {
        return Err(ObsError::AlreadyInstalled);
    }

    let builder = PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("latency_seconds".to_string()),
            &[0.001, 0.01, 0.1, 1.0, 10.0],
        )
        .unwrap();

    let handle = builder
        .install_recorder()
        .map_err(|_| ObsError::AlreadyInstalled)?;
    let _ = HANDLE.set(handle);
    Ok(HANDLE.get().unwrap())
}

/// Return installed handle if present.
pub fn handle() -> Option<&'static PrometheusHandle> {
    HANDLE.get()
}

pub fn inc_orders_submitted() {
    counter!("orders_submitted_total");
}

pub fn inc_orders_rejected() {
    counter!("orders_rejected_total");
}

pub fn inc_exec_errors() {
    counter!("exec_errors_total");
}

pub fn inc_metrics_scrapes() {
    counter!("metrics_scrapes_total");
}

// Gauges postponed for now (metrics 0.22 macro signature mismatch in your build).
pub fn set_open_positions(_n: u64) {}
