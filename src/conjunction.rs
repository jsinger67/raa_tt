use crate::proposition::Proposition;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Conjunction {
    pub left: Box<Proposition>,
    pub right: Box<Proposition>,
}

impl Display for Conjunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "({} & {})", self.left, self.right)
    }
}
