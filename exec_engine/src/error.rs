#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecError {
    Overfill,
    AlreadyFilled,
    InvalidTransition,
}
