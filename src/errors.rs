use thiserror::Error;

pub type Result<T> = std::result::Result<T, RaaError>;

#[derive(Error, Debug)]
pub enum RaaError {
    #[error("Tried to use a void expression (internal error)")]
    VoidExpression,
}
