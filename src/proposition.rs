use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Error, Formatter},
};

use crate::{
    bi_implication::BiImplication, conjunction::Conjunction, disjunction::Disjunction,
    implication::Implication, negation::Negation,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Proposition {
    #[default]
    Void,
    Atom(String),
    Negation(Negation),
    Implication(Implication),
    BiImplication(BiImplication),
    Disjunction(Disjunction),
    Conjunction(Conjunction),
}
impl Proposition {
    pub(crate) fn get_variables(&self) -> BTreeSet<String> {
        let mut vars = BTreeSet::new();
        self.inner_get_variables(&mut vars);
        vars
    }

    fn inner_get_variables(&self, vars: &mut BTreeSet<String>) {
        match self {
            Proposition::Void => (),
            Proposition::Atom(v) => {
                vars.insert(v.clone());
            }
            Proposition::Negation(Negation { inner }) => inner.inner_get_variables(vars),
            Proposition::Implication(Implication { left, right }) => {
                left.inner_get_variables(vars);
                right.inner_get_variables(vars);
            }
            Proposition::BiImplication(BiImplication { left, right }) => {
                left.inner_get_variables(vars);
                right.inner_get_variables(vars);
            }
            Proposition::Disjunction(Disjunction { left, right }) => {
                left.inner_get_variables(vars);
                right.inner_get_variables(vars);
            }
            Proposition::Conjunction(Conjunction { left, right }) => {
                left.inner_get_variables(vars);
                right.inner_get_variables(vars);
            }
        }
    }
}

impl Display for Proposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match self {
            Proposition::Void => write!(f, "()"),
            Proposition::Atom(v) => write!(f, "{}", v),
            Proposition::Negation(e) => write!(f, "{}", e),
            Proposition::Implication(i) => write!(f, "{}", i),
            Proposition::BiImplication(b) => write!(f, "{}", b),
            Proposition::Disjunction(d) => write!(f, "{}", d),
            Proposition::Conjunction(c) => write!(f, "{}", c),
        }
    }
}

impl From<&str> for Proposition {
    fn from(value: &str) -> Self {
        Proposition::Atom(value.to_owned())
    }
}

impl From<&crate::raa_tt_grammar_trait::RaaTtList<'_>> for Proposition {
    fn from(value: &crate::raa_tt_grammar_trait::RaaTtList<'_>) -> Self {
        (&value.biconditional).into()
    }
}

impl From<&crate::raa_tt_grammar_trait::Biconditional<'_>> for Proposition {
    fn from(value: &crate::raa_tt_grammar_trait::Biconditional<'_>) -> Self {
        let crate::raa_tt_grammar_trait::Biconditional {
            ref conditional,
            ref biconditional_list,
        } = value;
        if biconditional_list.is_empty() {
            conditional.into()
        } else {
            biconditional_list
                .iter()
                .fold(conditional.into(), |left: Proposition, b| {
                    Proposition::BiImplication(BiImplication {
                        left: Box::new(left),
                        right: Box::new((&b.conditional).into()),
                    })
                })
        }
    }
}

impl From<&crate::raa_tt_grammar_trait::Conditional<'_>> for Proposition {
    fn from(value: &crate::raa_tt_grammar_trait::Conditional<'_>) -> Self {
        let crate::raa_tt_grammar_trait::Conditional {
            ref disjunction,
            ref conditional_list,
        } = value;
        if conditional_list.is_empty() {
            disjunction.into()
        } else {
            conditional_list
                .iter()
                .fold(disjunction.into(), |left: Proposition, b| {
                    Proposition::Implication(Implication {
                        left: Box::new(left),
                        right: Box::new((&b.disjunction).into()),
                    })
                })
        }
    }
}

impl From<&crate::raa_tt_grammar_trait::Disjunction<'_>> for Proposition {
    fn from(value: &crate::raa_tt_grammar_trait::Disjunction<'_>) -> Self {
        let crate::raa_tt_grammar_trait::Disjunction {
            ref conjunction,
            ref disjunction_list,
        } = value;
        if disjunction_list.is_empty() {
            conjunction.into()
        } else {
            disjunction_list
                .iter()
                .fold(conjunction.into(), |left: Proposition, b| {
                    Proposition::Disjunction(Disjunction {
                        left: Box::new(left),
                        right: Box::new((&b.conjunction).into()),
                    })
                })
        }
    }
}

impl From<&crate::raa_tt_grammar_trait::Conjunction<'_>> for Proposition {
    fn from(value: &crate::raa_tt_grammar_trait::Conjunction<'_>) -> Self {
        let crate::raa_tt_grammar_trait::Conjunction {
            ref factor,
            ref conjunction_list,
        } = value;
        if conjunction_list.is_empty() {
            factor.into()
        } else {
            conjunction_list
                .iter()
                .fold(factor.into(), |left: Proposition, b| {
                    Proposition::Conjunction(Conjunction {
                        left: Box::new(left),
                        right: Box::new((&b.factor).into()),
                    })
                })
        }
    }
}

impl From<&crate::raa_tt_grammar_trait::Factor<'_>> for Proposition {
    fn from(value: &crate::raa_tt_grammar_trait::Factor<'_>) -> Self {
        match value {
            crate::raa_tt_grammar_trait::Factor::Var(crate::raa_tt_grammar_trait::FactorVar {
                var,
            }) => Proposition::Atom(var.var.text().to_owned()),
            crate::raa_tt_grammar_trait::Factor::Negation(f) => Proposition::Negation(Negation {
                inner: Box::new((&f.negation.factor).into()),
            }),
            crate::raa_tt_grammar_trait::Factor::LParBiconditionalRPar(b) => {
                (&*b.biconditional).into()
            }
        }
    }
}
