use std::sync::OnceLock;

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

    // Build recorder + handle
    let builder = PrometheusBuilder::new()
        // keep output small / deterministic-ish))
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
