use std::{ cell::RefCell, collections::BTreeMap };

use crate::{
    bi_implication::BiImplication,
    conjunction::Conjunction,
    disjunction::Disjunction,
    errors::{ RaaError, Result },
    implication::Implication,
    negation::Negation,
    proposition::Proposition,
    truth_table::TruthTable,
};

impl Proposition {
    fn calculate_value(&self, vars: &BTreeMap<String, bool>) -> Result<bool> {
        let result = match self {
            Proposition::Void => Err(RaaError::VoidExpression)?,
            Proposition::Atom(a) =>
                vars.get(a).ok_or(RaaError::UndefinedVariable { name: a.to_owned() }).copied()?,
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
    // Variable set paired with values that change in every line
    vars: RefCell<BTreeMap<String, bool>>,
}

impl TableGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_truth_table(&self, proposition: &Proposition) -> Result<TruthTable> {
        let vars = proposition.get_variables();
        *self.vars.borrow_mut() = vars.iter().fold(BTreeMap::new(), |mut acc, v| {
            acc.insert(v.clone(), false);
            acc
        });
        if self.number_of_vars() > 16 {
            return Err(RaaError::TooManyVariables);
        }
        let header = self.generate_table_header(proposition);
        let line_count = (2usize).pow(self.number_of_vars());
        let lines = Vec::with_capacity(line_count);
        let lines = (0..line_count).try_fold(lines, |mut lines, v| {
            self.generate_table_line(v, proposition).map(|line| {
                lines.push(line);
                lines
            })
        })?;
        Ok(TruthTable { header, lines })
    }

    fn generate_table_header(&self, proposition: &Proposition) -> Vec<String> {
        let mut header = self.vars
            .borrow()
            .keys()
            .fold(Vec::with_capacity(self.vars.borrow().len() + 1), |mut acc, var| {
                acc.push(var.clone());
                acc
            });
        header.push(proposition.to_string());
        header
    }

    #[inline]
    fn number_of_vars(&self) -> u32 {
        self.vars.borrow().len() as u32
    }

    fn generate_table_line(&self, v: usize, proposition: &Proposition) -> Result<Vec<bool>> {
        let n = self.number_of_vars();
        let mut bit = if n == 0 { 0 } else { 1 << (n - 1) };
        let mut line = self.vars
            .borrow_mut()
            .iter_mut()
            .fold(Vec::new(), |mut acc, (_var, val)| {
                // Extract the variable value from the bits of number v which is increased for
                // each line outside.
                let b = (v & bit) != 0;
                *val = b;
                acc.push(b);
                bit >>= 1;
                acc
            });
        let b = proposition.calculate_value(&self.vars.borrow())?;
        line.push(b);
        Ok(line)
    }
}
