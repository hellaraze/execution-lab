use super::common::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuditEvent {
    pub ts: Timestamp,
    pub source: &'static str,
    pub message: &'static str,
}

impl ContractEvent for AuditEvent {}
