use crate::proposition::Proposition;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Disjunction {
    pub(crate) left: Box<Proposition>,
    pub(crate) right: Box<Proposition>,
}

impl Display for Disjunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "({} | {})", self.left, self.right)
    }
}
