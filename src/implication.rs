use crate::proposition::Proposition;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
/// Implication struct represents a binary logical implication operation.
pub struct Implication {
    pub left: Box<Proposition>,
    pub right: Box<Proposition>,
}

impl Display for Implication {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "({} -> {})", self.left, self.right)
    }
}
