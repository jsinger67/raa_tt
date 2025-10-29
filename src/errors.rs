use thiserror::Error;

pub type Result<T> = std::result::Result<T, RaaError>;

#[derive(Error, Debug)]
pub enum RaaError {
    #[error("Tried to use a void expression (internal error)")]
    VoidExpression,
    #[error(
        "Too many variables for truth table generation.\n\
        Current: {current} variables, Maximum allowed: {max} variables.\n\
        \n\
        A truth table with {current} variables would require {rows} rows, consuming approximately {memory_mb:.1}MB of memory.\n\
        \n\
        Suggestions:\n\
        • Break down complex propositions into smaller sub-expressions\n\
        • Use logical equivalences to simplify the formula\n\
        • Consider using a SAT solver for propositions with many variables\n\
        • For educational purposes, focus on propositions with fewer variables"
    )]
    TooManyVariables {
        current: usize,
        max: usize,
        rows: u64,
        memory_mb: f64,
    },
    #[error("Variable {name} not defined")]
    UndefinedVariable { name: String },
    #[error(transparent)]
    FormatError { source: std::fmt::Error },
}
