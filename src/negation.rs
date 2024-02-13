use crate::proposition::Proposition;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
/// Negation struct represents an unary logical negation operation.
pub struct Negation {
    pub inner: Box<Proposition>,
}

impl Display for Negation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "!{}", self.inner)
    }
}
