use thiserror::Error;

pub type Result<T> = std::result::Result<T, RaaError>;

#[derive(Error, Debug)]
pub enum RaaError {
    #[error("Tried to use a void expression (internal error)")]
    VoidExpression,
    #[error(
        "Too many variables for truth table generation. Max allowed number is {1}. Current:{0}"
    )]
    TooManyVariables(usize, usize), // (current, max)
    #[error("Variable {name} not defined")]
    UndefinedVariable { name: String },
    #[error(transparent)]
    FormatError { source: std::fmt::Error },
}
