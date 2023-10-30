use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, RaaError>;

#[derive(Error, Debug)]
pub(crate) enum RaaError {
    #[error("No valid parse result available")]
    NoParseResult,
    #[error("Tried to use a void expression (internal error)")]
    VoidExpression,
    #[error(transparent)]
    FormatError { source: std::fmt::Error },
}
