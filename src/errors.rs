use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, RaaError>;

#[derive(Error, Debug)]
pub(crate) enum RaaError {
    #[error("Tried to use a void expression (internal error)")]
    VoidExpression,
}
