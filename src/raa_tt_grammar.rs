use crate::raa_tt_grammar_trait::{RaaTt, RaaTtGrammarTrait};
#[allow(unused_imports)]
use parol_runtime::{Result, Token};
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure that implements the semantic actions for our RaaTt grammar
/// !Change this type as needed!
///
#[derive(Debug, Default)]
pub struct RaaTtGrammar<'t> {
    pub raa_tt: Option<RaaTt<'t>>,
}

impl RaaTtGrammar<'_> {
    pub fn new() -> Self {
        RaaTtGrammar::default()
    }
}

impl Display for RaaTt<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for RaaTtGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.raa_tt {
            Some(raa_tt) => writeln!(f, "{}", raa_tt),
            None => write!(f, "No parse result"),
        }
    }
}

impl<'t> RaaTtGrammarTrait<'t> for RaaTtGrammar<'t> {
    // !Adjust your implementation as needed!

    /// Semantic action for non-terminal 'RaaTt'
    fn raa_tt(&mut self, arg: &RaaTt<'t>) -> Result<()> {
        self.raa_tt = Some(arg.clone());
        Ok(())
    }
}
