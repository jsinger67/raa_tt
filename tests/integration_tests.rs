use anyhow::Result;
use raa_tt::{
    conjunction::Conjunction,
    disjunction::Disjunction,
    errors::RaaError,
    proposition::Proposition,
    prover::{ProveResult, Prover},
    raa_tt_grammar::RaaTtGrammar,
    raa_tt_parser::parse,
    table_generator::TableGenerator,
};

pub const PROPOSITIONS: &[(&str, ProveResult)] = &[
    ("(p -> (!p & r))", ProveResult::Contingent),
    ("!p -> q", ProveResult::Contingent),
    ("p | q | !p", ProveResult::Proven),
    ("a -> b -> c -> d", ProveResult::Contingent),
    ("((p & q) -> p)", ProveResult::Proven),
    ("((p | q) -> (r -> p))", ProveResult::Contingent),
    ("p -> (q -> r | p)", ProveResult::Proven),
    ("p & q -> p", ProveResult::Proven),
    ("(p -> q) -> ((q -> r) -> (p -> r))", ProveResult::Proven),
    (
        "!((!p | !q) & (p <-> !r) -> !(q & r))",
        ProveResult::Contingent,
    ),
    ("p & q <-> !(p & q) | !q", ProveResult::Falsified),
    ("p & !p", ProveResult::Falsified),
    ("p | !p", ProveResult::Proven),
    (
        "(!p | !q) & (p <-> !r) -> !(q & r)",
        ProveResult::Contingent,
    ),
    ("!(p & q <-> !(p & q) | !q)", ProveResult::Proven),
    ("q & (p -> q) -> p", ProveResult::Contingent),
    ("p -> q -> (p & r -> p)", ProveResult::Proven),
    ("p & (!q -> !p) -> q", ProveResult::Proven),
    ("p & (p -> q) -> q", ProveResult::Proven),
    ("!q & (p -> q) -> !p", ProveResult::Proven),
    ("(p | q) & !p -> q", ProveResult::Proven),
    ("(p | q) & !q -> p", ProveResult::Proven),
    ("p -> (p | q)", ProveResult::Proven),
    ("p -> (q | p)", ProveResult::Proven),
    ("p & q -> p", ProveResult::Proven),
    ("p & q -> q", ProveResult::Proven),
    ("!!p -> p", ProveResult::Proven),
    ("((p -> q) & (q -> r)) -> (p -> r)", ProveResult::Proven),
    (
        "(p -> (q -> r)) -> ((p -> q) -> (p -> r))",
        ProveResult::Proven,
    ),
    (
        "((p -> q) & (r -> s) & (p | r)) -> q | s",
        ProveResult::Proven,
    ),
    (
        "(p -> q) & (r -> s) & (!q | !s) -> !p | !r",
        ProveResult::Proven,
    ),
    ("p -> p", ProveResult::Proven),
    ("!p -> (p -> q)", ProveResult::Proven),
    ("q -> (p -> q)", ProveResult::Proven),
    ("p & !p -> q", ProveResult::Proven),
    ("p -> (q | !q)", ProveResult::Proven),
];

#[test]
fn compile_and_prove() -> Result<()> {
    for (i, (p, r)) in PROPOSITIONS.iter().enumerate() {
        let mut raa_tt_grammar = RaaTtGrammar::new();
        let file_name = format!("example_{i}");
        parse(p, &file_name, &mut raa_tt_grammar)?;
        assert_eq!(1, raa_tt_grammar.raa_tt.as_ref().unwrap().raa_tt_list.len());
        let proposition: Proposition =
            (&raa_tt_grammar.raa_tt.as_ref().unwrap().raa_tt_list[0].biconditional).into();
        let solver = Prover::new();
        let solve_result = solver.prove(&proposition)?;
        assert_eq!(*r, solve_result);
    }
    Ok(())
}

// Helper functions for variable count testing

/// Creates a proposition with exactly `count` unique variables using a chain of conjunctions.
/// Variables are named "var0", "var1", ..., "varN"
fn create_proposition_with_variables(count: usize) -> Proposition {
    if count == 0 {
        // Return a simple atomic proposition for zero variables case
        return Proposition::Atom("a".to_string());
    }

    let mut proposition = Proposition::Atom("var0".to_string());

    for i in 1..count {
        let next_var = Proposition::Atom(format!("var{}", i));
        proposition = Proposition::Conjunction(Conjunction {
            left: Box::new(proposition),
            right: Box::new(next_var),
        });
    }

    proposition
}

/// Creates a proposition with exactly `count` unique variables using a chain of disjunctions.
/// This provides an alternative structure for testing different proposition types.
fn create_disjunctive_proposition_with_variables(count: usize) -> Proposition {
    if count == 0 {
        return Proposition::Atom("a".to_string());
    }

    let mut proposition = Proposition::Atom("x0".to_string());

    for i in 1..count {
        let next_var = Proposition::Atom(format!("x{}", i));
        proposition = Proposition::Disjunction(Disjunction {
            left: Box::new(proposition),
            right: Box::new(next_var),
        });
    }

    proposition
}

// Variable count validation tests

#[test]
fn test_variable_count_zero_variables() -> Result<()> {
    let generator = TableGenerator::new();

    // Test with a simple atom (1 variable, not 0)
    let proposition = Proposition::Atom("p".to_string());
    let result = generator.generate_truth_table(&proposition);
    assert!(result.is_ok());

    let truth_table = result.unwrap();
    assert_eq!(truth_table.header.len(), 2); // 1 variable + 1 proposition column
    assert_eq!(truth_table.lines.len(), 2); // 2^1 = 2 rows

    Ok(())
}

#[test]
fn test_variable_count_one_variable() -> Result<()> {
    let generator = TableGenerator::new();
    let proposition = create_proposition_with_variables(1);

    let result = generator.generate_truth_table(&proposition);
    assert!(result.is_ok());

    let truth_table = result.unwrap();
    assert_eq!(truth_table.lines.len(), 2); // 2^1 = 2 rows

    Ok(())
}

#[test]
fn test_variable_count_warning_threshold() -> Result<()> {
    let generator = TableGenerator::new();

    // Test exactly at warning threshold (12 variables)
    let proposition = create_proposition_with_variables(12);
    let result = generator.generate_truth_table(&proposition);
    assert!(result.is_ok());

    let truth_table = result.unwrap();
    assert_eq!(truth_table.lines.len(), 4096); // 2^12 = 4096 rows

    Ok(())
}

#[test]
fn test_variable_count_above_warning_threshold() -> Result<()> {
    let generator = TableGenerator::new();

    // Test above warning threshold (13 variables) - should still succeed
    let proposition = create_proposition_with_variables(13);
    let result = generator.generate_truth_table(&proposition);
    assert!(result.is_ok());

    let truth_table = result.unwrap();
    assert_eq!(truth_table.lines.len(), 8192); // 2^13 = 8192 rows

    Ok(())
}

#[test]
fn test_variable_count_near_maximum() -> Result<()> {
    let generator = TableGenerator::new();

    // Test with 15 variables (just under the limit)
    let proposition = create_proposition_with_variables(15);
    let result = generator.generate_truth_table(&proposition);
    assert!(result.is_ok());

    let truth_table = result.unwrap();
    assert_eq!(truth_table.lines.len(), 32768); // 2^15 = 32768 rows

    Ok(())
}

#[test]
fn test_variable_count_exactly_maximum() -> Result<()> {
    let generator = TableGenerator::new();

    // Test with exactly 16 variables (at the limit)
    let proposition = create_proposition_with_variables(16);
    let result = generator.generate_truth_table(&proposition);
    assert!(result.is_ok());

    let truth_table = result.unwrap();
    assert_eq!(truth_table.lines.len(), 65536); // 2^16 = 65536 rows
    assert_eq!(truth_table.header.len(), 17); // 16 variables + 1 proposition column

    Ok(())
}

#[test]
fn test_variable_count_exceeds_maximum() -> Result<()> {
    let generator = TableGenerator::new();

    // Test with 17 variables (exceeds the limit)
    let proposition = create_proposition_with_variables(17);
    let result = generator.generate_truth_table(&proposition);

    assert!(result.is_err());

    match result.unwrap_err() {
        RaaError::TooManyVariables {
            current,
            max,
            rows,
            memory_mb,
        } => {
            assert_eq!(current, 17);
            assert_eq!(max, 16);
            assert_eq!(rows, 131072); // 2^17 = 131072 rows
            assert!(memory_mb > 0.0);
        }
        other => panic!("Expected TooManyVariables error, got: {:?}", other),
    }

    Ok(())
}

#[test]
fn test_variable_count_significantly_exceeds_maximum() -> Result<()> {
    let generator = TableGenerator::new();

    // Test with 20 variables (significantly exceeds the limit)
    let proposition = create_proposition_with_variables(20);
    let result = generator.generate_truth_table(&proposition);

    assert!(result.is_err());

    match result.unwrap_err() {
        RaaError::TooManyVariables {
            current,
            max,
            rows,
            memory_mb,
        } => {
            assert_eq!(current, 20);
            assert_eq!(max, 16);
            assert_eq!(rows, 1048576); // 2^20 = 1048576 rows
            assert!(memory_mb > 1.0); // Should be significant memory usage
        }
        other => panic!("Expected TooManyVariables error, got: {:?}", other),
    }

    Ok(())
}

#[test]
fn test_variable_count_extreme_case() -> Result<()> {
    let generator = TableGenerator::new();

    // Test with 25 variables (extreme case)
    let proposition = create_proposition_with_variables(25);
    let result = generator.generate_truth_table(&proposition);

    assert!(result.is_err());

    match result.unwrap_err() {
        RaaError::TooManyVariables {
            current,
            max,
            rows,
            memory_mb,
        } => {
            assert_eq!(current, 25);
            assert_eq!(max, 16);
            assert_eq!(rows, 33554432); // 2^25 = 33554432 rows
            assert!(memory_mb > 500.0); // Should be very high memory usage
        }
        other => panic!("Expected TooManyVariables error, got: {:?}", other),
    }

    Ok(())
}

#[test]
fn test_error_message_content() -> Result<()> {
    let generator = TableGenerator::new();
    let proposition = create_proposition_with_variables(18);
    let result = generator.generate_truth_table(&proposition);

    assert!(result.is_err());

    let error_message = result.unwrap_err().to_string();

    // Verify the error message contains expected content
    assert!(error_message.contains("Too many variables"));
    assert!(error_message.contains("Current: 18 variables"));
    assert!(error_message.contains("Maximum allowed: 16 variables"));
    assert!(error_message.contains("262144 rows")); // 2^18
    assert!(error_message.contains("Break down complex propositions"));
    assert!(error_message.contains("Use logical equivalences"));
    assert!(error_message.contains("SAT solver"));

    Ok(())
}

#[test]
fn test_different_proposition_structures() -> Result<()> {
    let generator = TableGenerator::new();

    // Test conjunctive structure at the limit
    let conjunctive_prop = create_proposition_with_variables(16);
    let result1 = generator.generate_truth_table(&conjunctive_prop);
    assert!(result1.is_ok());

    // Test disjunctive structure at the limit
    let disjunctive_prop = create_disjunctive_proposition_with_variables(16);
    let result2 = generator.generate_truth_table(&disjunctive_prop);
    assert!(result2.is_ok());

    // Both should have the same number of rows
    assert_eq!(result1.unwrap().lines.len(), result2.unwrap().lines.len());

    Ok(())
}

#[test]
fn test_different_proposition_structures_exceeding_limit() -> Result<()> {
    let generator = TableGenerator::new();

    // Test conjunctive structure exceeding the limit
    let conjunctive_prop = create_proposition_with_variables(17);
    let result1 = generator.generate_truth_table(&conjunctive_prop);
    assert!(result1.is_err());

    // Test disjunctive structure exceeding the limit
    let disjunctive_prop = create_disjunctive_proposition_with_variables(17);
    let result2 = generator.generate_truth_table(&disjunctive_prop);
    assert!(result2.is_err());

    // Both should fail with TooManyVariables error
    match (result1.unwrap_err(), result2.unwrap_err()) {
        (
            RaaError::TooManyVariables { current: c1, .. },
            RaaError::TooManyVariables { current: c2, .. },
        ) => {
            assert_eq!(c1, 17);
            assert_eq!(c2, 17);
        }
        (e1, e2) => panic!("Expected TooManyVariables errors, got: {:?}, {:?}", e1, e2),
    }

    Ok(())
}

#[test]
fn test_memory_calculation_accuracy() -> Result<()> {
    let generator = TableGenerator::new();
    let proposition = create_proposition_with_variables(17);
    let result = generator.generate_truth_table(&proposition);

    assert!(result.is_err());

    match result.unwrap_err() {
        RaaError::TooManyVariables {
            current,
            max,
            rows,
            memory_mb,
        } => {
            // Verify calculations are correct
            assert_eq!(current, 17);
            assert_eq!(max, 16);
            assert_eq!(rows, 131072); // 2^17

            // Expected memory: 131072 rows * 18 booleans per row (17 vars + 1 result) / (1024*1024)
            let expected_memory = (131072.0 * 18.0) / (1024.0 * 1024.0);
            assert!((memory_mb - expected_memory).abs() < 0.1); // Allow small floating point differences
        }
        other => panic!("Expected TooManyVariables error, got: {:?}", other),
    }

    Ok(())
}
