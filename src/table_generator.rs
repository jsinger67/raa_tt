use std::{cell::RefCell, collections::BTreeMap, fmt::Write};

use crate::{
    bi_implication::BiImplication,
    conjunction::Conjunction,
    disjunction::Disjunction,
    errors::{RaaError, Result},
    implication::Implication,
    negation::Negation,
    proposition::Proposition,
};

impl Proposition {
    fn calculate_value(&self, vars: &BTreeMap<String, bool>) -> Result<bool> {
        let result = match self {
            Proposition::Void => Err(RaaError::VoidExpression)?,
            Proposition::Atom(a) => vars
                .get(a)
                .ok_or(RaaError::UndefinedVariable { name: a.to_owned() })
                .map(|b| *b)?,
            Proposition::Negation(Negation { inner }) => !inner.calculate_value(vars)?,
            Proposition::Implication(Implication { left, right }) => {
                !left.calculate_value(vars)? || right.calculate_value(vars)?
            }
            Proposition::BiImplication(BiImplication { left, right }) => {
                left.calculate_value(vars)? == right.calculate_value(vars)?
            }
            Proposition::Disjunction(Disjunction { left, right }) => {
                left.calculate_value(vars)? || right.calculate_value(vars)?
            }
            Proposition::Conjunction(Conjunction { left, right }) => {
                left.calculate_value(vars)? && right.calculate_value(vars)?
            }
        };
        Ok(result)
    }
}

#[derive(Debug, Default)]
pub struct TableGenerator {
    vars: RefCell<BTreeMap<String, bool>>,
}

impl TableGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_truth_table(&self, proposition: &Proposition) -> Result<()> {
        let vars = proposition.get_variables();
        *self.vars.borrow_mut() = vars.iter().fold(BTreeMap::new(), |mut acc, v| {
            acc.insert(v.clone(), false);
            acc
        });
        if self.number_of_vars() > 16 {
            return Err(RaaError::TooManyVariables);
        }
        let var_part = self.generate_table_header()?;
        let prp_part = proposition.to_string();
        println!("{var_part} {prp_part}");
        println!("{}", "-".repeat(var_part.len() + prp_part.len()));
        for v in 0..2usize.pow(self.number_of_vars()) {
            self.generate_table_line(v, proposition)?;
        }
        Ok(())
    }

    fn generate_table_header(&self) -> Result<String> {
        let mut line = String::new();
        self.vars.borrow().keys().try_for_each(|var| {
            write!(line, "{var} | ").map_err(|e| RaaError::FormatError { source: e })
        })?;
        Ok(line)
    }

    #[inline]
    fn number_of_vars(&self) -> u32 {
        self.vars.borrow().len() as u32
    }

    fn generate_table_line(&self, v: usize, proposition: &Proposition) -> Result<()> {
        let n = self.number_of_vars();
        let mut bit = if n == 0 { 0 } else { 1 << (n - 1) };
        self.vars.borrow_mut().iter_mut().for_each(|(var, val)| {
            // Extract the variable value from the bits of number v which is increased for each line
            let b = v & bit != 0;
            *val = b;
            let t = if b { "T" } else { "F" };
            print!("{t:w$} | ", w = var.len());
            bit >>= 1;
        });
        let b = proposition.calculate_value(&self.vars.borrow())?;
        let t = if b { "T" } else { "F" };
        print!(" {t}");
        println!();
        Ok(())
    }
}
