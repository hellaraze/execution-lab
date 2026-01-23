#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecError {
    NotFound,
    ConfigMismatch,
    Overfill,
    AlreadyFilled,
    InvalidTransition,
}
