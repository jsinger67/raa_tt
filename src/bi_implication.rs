use crate::proposition::Proposition;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct BiImplication {
    pub(crate) left: Box<Proposition>,
    pub(crate) right: Box<Proposition>,
}

impl Display for BiImplication {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "({} <-> {})", self.left, self.right)
    }
}
