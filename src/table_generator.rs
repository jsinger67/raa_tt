use std::{cell::RefCell, collections::BTreeMap};

use crate::{
    bi_implication::BiImplication,
    conjunction::Conjunction,
    disjunction::Disjunction,
    errors::{RaaError, Result},
    implication::Implication,
    negation::Negation,
    proposition::Proposition,
    truth_table::TruthTable,
};

impl Proposition {
    fn calculate_value(&self, vars: &BTreeMap<String, bool>) -> Result<bool> {
        let result = match self {
            Proposition::Void => Err(RaaError::VoidExpression)?,
            Proposition::Atom(a) => vars
                .get(a)
                .ok_or(RaaError::UndefinedVariable { name: a.to_owned() })
                .copied()?,
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
    /// Creates a new instance of the TableGenerator.
    ///
    /// This initializes the generator with an empty state, ready to create truth tables
    /// for logical propositions. The generator maintains an internal variable mapping
    /// that gets populated during truth table generation.
    ///
    /// # Examples
    ///
    /// ```
    /// use raa_tt::table_generator::TableGenerator;
    ///
    /// let generator = TableGenerator::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates a complete truth table for a logical proposition.
    ///
    /// This method creates a comprehensive truth table that shows all possible truth value
    /// assignments for the variables in the given proposition and the resulting truth value
    /// of the entire proposition for each assignment.
    ///
    /// ## Algorithm Overview
    ///
    /// The truth table generation process works by:
    /// 1. **Variable Extraction**: Identifying all unique propositional variables in the formula
    /// 2. **Truth Value Enumeration**: Systematically generating all possible truth value assignments
    /// 3. **Proposition Evaluation**: Computing the truth value of the proposition for each assignment
    /// 4. **Table Construction**: Organizing results into a structured table format
    ///
    /// The algorithm uses a binary counting approach where each row represents a unique
    /// combination of variable assignments. For n variables, this generates exactly 2^n rows.
    ///
    /// ## Complexity Analysis
    ///
    /// - **Time Complexity**: O(2^n × m) where n is the number of variables and m is the
    ///   complexity of evaluating the proposition. Each of the 2^n truth value assignments
    ///   requires evaluating the entire proposition structure.
    ///
    /// - **Space Complexity**: O(2^n × n) for storing the complete truth table, where each
    ///   row contains n variable values plus the proposition result.
    ///
    /// ## Variable Limit
    ///
    /// Truth table generation is limited to propositions with at most 16 variables to prevent
    /// excessive memory usage and computation time. This limit ensures reasonable performance
    /// while still supporting complex logical formulas.
    ///
    /// ## Examples
    ///
    /// ### Simple Proposition
    /// ```
    /// use raa_tt::{
    ///     table_generator::TableGenerator,
    ///     proposition::Proposition,
    ///     conjunction::Conjunction,
    /// };
    ///
    /// // Generate truth table for: P ∧ Q
    /// let generator = TableGenerator::new();
    /// let proposition = Proposition::Conjunction(Conjunction {
    ///     left: Box::new("P".into()),
    ///     right: Box::new("Q".into()),
    /// });
    ///
    /// let truth_table = generator.generate_truth_table(&proposition).unwrap();
    /// assert_eq!(truth_table.header, vec!["P", "Q", "(P & Q)"]);
    /// assert_eq!(truth_table.lines.len(), 4); // 2^2 = 4 rows
    ///
    /// // Expected rows: [F,F,F], [F,T,F], [T,F,F], [T,T,T]
    /// assert_eq!(truth_table.lines[0], vec![false, false, false]);
    /// assert_eq!(truth_table.lines[3], vec![true, true, true]);
    /// ```
    ///
    /// ### Complex Proposition with Implication
    /// ```
    /// use raa_tt::{
    ///     table_generator::TableGenerator,
    ///     proposition::Proposition,
    ///     implication::Implication,
    ///     disjunction::Disjunction,
    /// };
    ///
    /// // Generate truth table for: (P ∨ Q) → P
    /// let generator = TableGenerator::new();
    /// let proposition = Proposition::Implication(Implication {
    ///     left: Box::new(Proposition::Disjunction(Disjunction {
    ///         left: Box::new("P".into()),
    ///         right: Box::new("Q".into()),
    ///     })),
    ///     right: Box::new("P".into()),
    /// });
    ///
    /// let truth_table = generator.generate_truth_table(&proposition).unwrap();
    /// assert_eq!(truth_table.header, vec!["P", "Q", "((P | Q) -> P)"]);
    /// assert_eq!(truth_table.lines.len(), 4);
    /// ```
    ///
    /// ### Tautology Detection
    /// ```
    /// use raa_tt::{
    ///     table_generator::TableGenerator,
    ///     proposition::Proposition,
    ///     disjunction::Disjunction,
    ///     negation::Negation,
    /// };
    ///
    /// // Generate truth table for: P ∨ ¬P (Law of Excluded Middle)
    /// let generator = TableGenerator::new();
    /// let proposition = Proposition::Disjunction(Disjunction {
    ///     left: Box::new("P".into()),
    ///     right: Box::new(Proposition::Negation(Negation {
    ///         inner: Box::new("P".into()),
    ///     })),
    /// });
    ///
    /// let truth_table = generator.generate_truth_table(&proposition).unwrap();
    ///
    /// // All rows should evaluate to true (tautology)
    /// assert!(truth_table.lines.iter().all(|row| row.last() == Some(&true)));
    /// ```
    ///
    /// ### Multiple Variables
    /// ```
    /// use raa_tt::{
    ///     table_generator::TableGenerator,
    ///     proposition::Proposition,
    ///     conjunction::Conjunction,
    ///     disjunction::Disjunction,
    /// };
    ///
    /// // Generate truth table for: (P ∧ Q) ∨ (R ∧ S)
    /// let generator = TableGenerator::new();
    /// let proposition = Proposition::Disjunction(Disjunction {
    ///     left: Box::new(Proposition::Conjunction(Conjunction {
    ///         left: Box::new("P".into()),
    ///         right: Box::new("Q".into()),
    ///     })),
    ///     right: Box::new(Proposition::Conjunction(Conjunction {
    ///         left: Box::new("R".into()),
    ///         right: Box::new("S".into()),
    ///     })),
    /// });
    ///
    /// let truth_table = generator.generate_truth_table(&proposition).unwrap();
    /// assert_eq!(truth_table.lines.len(), 16); // 2^4 = 16 rows for 4 variables
    /// ```
    ///
    /// ## Mathematical Context
    ///
    /// Truth tables are fundamental tools in propositional logic that provide a complete
    /// semantic characterization of logical formulas. They show:
    ///
    /// - **Tautologies**: Propositions that are true under all possible assignments
    /// - **Contradictions**: Propositions that are false under all possible assignments
    /// - **Contingencies**: Propositions whose truth value depends on variable assignments
    /// - **Logical Equivalence**: Two propositions with identical truth tables are logically equivalent
    ///
    /// The systematic enumeration of all possible truth value assignments ensures that
    /// every logical possibility is considered, making truth tables a complete decision
    /// procedure for propositional satisfiability.
    ///
    /// ## Return Value
    ///
    /// Returns a [`TruthTable`] containing:
    /// - `header`: Variable names followed by the proposition string representation
    /// - `lines`: Each row represents one truth value assignment with the final column
    ///   showing the proposition's truth value for that assignment
    ///
    /// ## Errors
    ///
    /// - [`RaaError::TooManyVariables`]: Returned when the proposition contains more than 16 variables
    /// - [`RaaError::VoidExpression`]: Returned when the proposition contains invalid expressions
    /// - [`RaaError::UndefinedVariable`]: Returned when variable evaluation fails (internal error)
    ///
    /// [`TruthTable`]: crate::truth_table::TruthTable
    /// [`RaaError::TooManyVariables`]: crate::errors::RaaError::TooManyVariables
    /// [`RaaError::VoidExpression`]: crate::errors::RaaError::VoidExpression
    /// [`RaaError::UndefinedVariable`]: crate::errors::RaaError::UndefinedVariable
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
        let mut header = self.vars.borrow().keys().fold(
            Vec::with_capacity(self.vars.borrow().len() + 1),
            |mut acc, var| {
                acc.push(var.clone());
                acc
            },
        );
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
        let mut line =
            self.vars
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
