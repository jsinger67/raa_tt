use crate::proposition::Proposition;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Negation {
    pub(crate) inner: Box<Proposition>,
}

impl Negation {
    pub(crate) fn is_negation_of(&self, other: &Proposition) -> bool {
        *self.inner == *other
    }
}

impl Display for Negation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "!{}", self.inner)
    }
}
