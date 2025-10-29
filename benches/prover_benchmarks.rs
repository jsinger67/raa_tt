//! Benchmarks for the Prover component
//!
//! This benchmark suite tests the performance of the Prover across different types of propositions
//! to detect performance regressions and understand performance characteristics.
//!
//! ## Running the benchmarks
//!
//! To run all prover benchmarks:
//! ```bash
//! cargo bench --bench prover_benchmarks
//! ```
//!
//! To run specific benchmark groups:
//! ```bash
//! cargo bench --bench prover_benchmarks -- simple_propositions
//! cargo bench --bench prover_benchmarks -- complex_propositions
//! cargo bench --bench prover_benchmarks -- tautologies
//! cargo bench --bench prover_benchmarks -- worst_case_scenarios
//! cargo bench --bench prover_benchmarks -- prove_result_variants
//! ```

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use raa_tt::{
    bi_implication::BiImplication,
    conjunction::Conjunction,
    disjunction::Disjunction,
    implication::Implication,
    negation::Negation,
    proposition::Proposition,
    prover::{ProveResult, Prover},
};

/// Simple propositions with basic operations
fn simple_propositions() -> Vec<(&'static str, Proposition)> {
    vec![
        // Single variables
        ("single_atom", Proposition::Atom("p".to_string())),
        // Basic negation
        (
            "simple_negation",
            Proposition::Negation(Negation {
                inner: Box::new(Proposition::Atom("p".to_string())),
            }),
        ),
        // Basic conjunction
        (
            "simple_conjunction",
            Proposition::Conjunction(Conjunction {
                left: Box::new(Proposition::Atom("p".to_string())),
                right: Box::new(Proposition::Atom("q".to_string())),
            }),
        ),
        // Basic disjunction
        (
            "simple_disjunction",
            Proposition::Disjunction(Disjunction {
                left: Box::new(Proposition::Atom("p".to_string())),
                right: Box::new(Proposition::Atom("q".to_string())),
            }),
        ),
        // Basic implication
        (
            "simple_implication",
            Proposition::Implication(Implication {
                left: Box::new(Proposition::Atom("p".to_string())),
                right: Box::new(Proposition::Atom("q".to_string())),
            }),
        ),
        // Basic biimplication
        (
            "simple_biimplication",
            Proposition::BiImplication(BiImplication {
                left: Box::new(Proposition::Atom("p".to_string())),
                right: Box::new(Proposition::Atom("q".to_string())),
            }),
        ),
    ]
}

/// Complex propositions with nested operations and multiple variables
fn complex_propositions() -> Vec<(&'static str, Proposition)> {
    vec![
        // Complex conjunction with implication
        (
            "complex_conjunction_implication",
            Proposition::Conjunction(Conjunction {
                left: Box::new(Proposition::Atom("p".to_string())),
                right: Box::new(Proposition::Implication(Implication {
                    left: Box::new(Proposition::Atom("p".to_string())),
                    right: Box::new(Proposition::Atom("q".to_string())),
                })),
            }),
        ),
        // Nested implications
        (
            "nested_implications",
            Proposition::Implication(Implication {
                left: Box::new(Proposition::Implication(Implication {
                    left: Box::new(Proposition::Atom("p".to_string())),
                    right: Box::new(Proposition::Atom("q".to_string())),
                })),
                right: Box::new(Proposition::Implication(Implication {
                    left: Box::new(Proposition::Atom("q".to_string())),
                    right: Box::new(Proposition::Atom("r".to_string())),
                })),
            }),
        ),
        // Complex formula with multiple operators
        (
            "complex_mixed_operators",
            Proposition::Implication(Implication {
                left: Box::new(Proposition::Conjunction(Conjunction {
                    left: Box::new(Proposition::Disjunction(Disjunction {
                        left: Box::new(Proposition::Atom("p".to_string())),
                        right: Box::new(Proposition::Atom("q".to_string())),
                    })),
                    right: Box::new(Proposition::Negation(Negation {
                        inner: Box::new(Proposition::Atom("r".to_string())),
                    })),
                })),
                right: Box::new(Proposition::BiImplication(BiImplication {
                    left: Box::new(Proposition::Atom("s".to_string())),
                    right: Box::new(Proposition::Atom("t".to_string())),
                })),
            }),
        ),
        // Four-variable complex formula
        (
            "four_variable_complex",
            Proposition::Implication(Implication {
                left: Box::new(Proposition::Conjunction(Conjunction {
                    left: Box::new(Proposition::Implication(Implication {
                        left: Box::new(Proposition::Atom("p".to_string())),
                        right: Box::new(Proposition::Atom("q".to_string())),
                    })),
                    right: Box::new(Proposition::Implication(Implication {
                        left: Box::new(Proposition::Atom("r".to_string())),
                        right: Box::new(Proposition::Atom("s".to_string())),
                    })),
                })),
                right: Box::new(Proposition::Disjunction(Disjunction {
                    left: Box::new(Proposition::Atom("q".to_string())),
                    right: Box::new(Proposition::Atom("s".to_string())),
                })),
            }),
        ),
    ]
}

/// Known tautologies from the test suite
fn tautologies() -> Vec<(&'static str, Proposition)> {
    vec![
        // Law of excluded middle: p | !p
        (
            "law_of_excluded_middle",
            Proposition::Disjunction(Disjunction {
                left: Box::new(Proposition::Atom("p".to_string())),
                right: Box::new(Proposition::Negation(Negation {
                    inner: Box::new(Proposition::Atom("p".to_string())),
                })),
            }),
        ),
        // Modus Ponens: (p & (p -> q)) -> q
        (
            "modus_ponens",
            Proposition::Implication(Implication {
                left: Box::new(Proposition::Conjunction(Conjunction {
                    left: Box::new(Proposition::Atom("p".to_string())),
                    right: Box::new(Proposition::Implication(Implication {
                        left: Box::new(Proposition::Atom("p".to_string())),
                        right: Box::new(Proposition::Atom("q".to_string())),
                    })),
                })),
                right: Box::new(Proposition::Atom("q".to_string())),
            }),
        ),
        // Hypothetical syllogism: ((p -> q) & (q -> r)) -> (p -> r)
        (
            "hypothetical_syllogism",
            Proposition::Implication(Implication {
                left: Box::new(Proposition::Conjunction(Conjunction {
                    left: Box::new(Proposition::Implication(Implication {
                        left: Box::new(Proposition::Atom("p".to_string())),
                        right: Box::new(Proposition::Atom("q".to_string())),
                    })),
                    right: Box::new(Proposition::Implication(Implication {
                        left: Box::new(Proposition::Atom("q".to_string())),
                        right: Box::new(Proposition::Atom("r".to_string())),
                    })),
                })),
                right: Box::new(Proposition::Implication(Implication {
                    left: Box::new(Proposition::Atom("p".to_string())),
                    right: Box::new(Proposition::Atom("r".to_string())),
                })),
            }),
        ),
    ]
}

/// Worst-case scenarios with deeply nested formulas
fn worst_case_scenarios() -> Vec<(&'static str, Proposition)> {
    vec![
        // Deeply nested conjunction (depth 5)
        ("deep_conjunction_5", create_deep_conjunction(5)),
        // Deeply nested disjunction (depth 5)
        ("deep_disjunction_5", create_deep_disjunction(5)),
        // Deeply nested implication chain (depth 4)
        ("deep_implication_4", create_deep_implication_chain(4)),
        // Complex nested with multiple branches
        (
            "complex_nested_branches",
            Proposition::Conjunction(Conjunction {
                left: Box::new(create_deep_disjunction(3)),
                right: Box::new(create_deep_conjunction(3)),
            }),
        ),
    ]
}

/// Propositions that result in different ProveResult variants
fn prove_result_variants() -> Vec<(&'static str, Proposition, ProveResult)> {
    vec![
        // Proven (tautology)
        (
            "proven_tautology",
            Proposition::Disjunction(Disjunction {
                left: Box::new(Proposition::Atom("p".to_string())),
                right: Box::new(Proposition::Negation(Negation {
                    inner: Box::new(Proposition::Atom("p".to_string())),
                })),
            }),
            ProveResult::Proven,
        ),
        // Falsified (contradiction)
        (
            "falsified_contradiction",
            Proposition::Conjunction(Conjunction {
                left: Box::new(Proposition::Atom("p".to_string())),
                right: Box::new(Proposition::Negation(Negation {
                    inner: Box::new(Proposition::Atom("p".to_string())),
                })),
            }),
            ProveResult::Falsified,
        ),
        // Contingent
        (
            "contingent_simple",
            Proposition::Atom("p".to_string()),
            ProveResult::Contingent,
        ),
        // Contingent with multiple variables
        (
            "contingent_complex",
            Proposition::Conjunction(Conjunction {
                left: Box::new(Proposition::Atom("p".to_string())),
                right: Box::new(Proposition::Atom("q".to_string())),
            }),
            ProveResult::Contingent,
        ),
    ]
}

/// Create a deeply nested conjunction
fn create_deep_conjunction(depth: usize) -> Proposition {
    if depth == 0 {
        return Proposition::Atom("base".to_string());
    }

    Proposition::Conjunction(Conjunction {
        left: Box::new(Proposition::Atom(format!("var{}", depth))),
        right: Box::new(create_deep_conjunction(depth - 1)),
    })
}

/// Create a deeply nested disjunction
fn create_deep_disjunction(depth: usize) -> Proposition {
    if depth == 0 {
        return Proposition::Atom("base".to_string());
    }

    Proposition::Disjunction(Disjunction {
        left: Box::new(Proposition::Atom(format!("var{}", depth))),
        right: Box::new(create_deep_disjunction(depth - 1)),
    })
}

/// Create a deeply nested implication chain
fn create_deep_implication_chain(depth: usize) -> Proposition {
    if depth == 0 {
        return Proposition::Atom("base".to_string());
    }

    Proposition::Implication(Implication {
        left: Box::new(Proposition::Atom(format!("var{}", depth))),
        right: Box::new(create_deep_implication_chain(depth - 1)),
    })
}

/// Benchmark simple propositions
fn bench_simple_propositions(c: &mut Criterion) {
    let mut group = c.benchmark_group("prover_simple_propositions");

    for (name, proposition) in simple_propositions() {
        group.bench_with_input(BenchmarkId::new("prove", name), &proposition, |b, prop| {
            b.iter(|| {
                let prover = Prover::new();
                prover.prove(prop).unwrap()
            });
        });
    }

    group.finish();
}

/// Benchmark complex propositions
fn bench_complex_propositions(c: &mut Criterion) {
    let mut group = c.benchmark_group("prover_complex_propositions");

    for (name, proposition) in complex_propositions() {
        group.bench_with_input(BenchmarkId::new("prove", name), &proposition, |b, prop| {
            b.iter(|| {
                let prover = Prover::new();
                prover.prove(prop).unwrap()
            });
        });
    }

    group.finish();
}

/// Benchmark known tautologies
fn bench_tautologies(c: &mut Criterion) {
    let mut group = c.benchmark_group("prover_tautologies");

    for (name, proposition) in tautologies() {
        group.bench_with_input(BenchmarkId::new("prove", name), &proposition, |b, prop| {
            b.iter(|| {
                let prover = Prover::new();
                prover.prove(prop).unwrap()
            });
        });
    }

    group.finish();
}

/// Benchmark worst-case scenarios
fn bench_worst_case_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("prover_worst_case_scenarios");

    for (name, proposition) in worst_case_scenarios() {
        group.bench_with_input(BenchmarkId::new("prove", name), &proposition, |b, prop| {
            b.iter(|| {
                let prover = Prover::new();
                prover.prove(prop).unwrap()
            });
        });
    }

    group.finish();
}

/// Benchmark propositions that result in different ProveResult variants
fn bench_prove_result_variants(c: &mut Criterion) {
    let mut group = c.benchmark_group("prover_prove_result_variants");

    for (name, proposition, expected_result) in prove_result_variants() {
        group.bench_with_input(
            BenchmarkId::new("prove", format!("{}_{:?}", name, expected_result)),
            &proposition,
            |b, prop| {
                b.iter(|| {
                    let prover = Prover::new();
                    let result = prover.prove(prop).unwrap();
                    // Verify we get the expected result
                    assert_eq!(result, expected_result);
                    result
                });
            },
        );
    }

    group.finish();
}

/// Benchmark prover creation overhead
fn bench_prover_creation(c: &mut Criterion) {
    c.bench_function("prover_creation", |b| {
        b.iter(|| Prover::new());
    });
}

criterion_group!(
    prover_benches,
    bench_prover_creation,
    bench_simple_propositions,
    bench_complex_propositions,
    bench_tautologies,
    bench_worst_case_scenarios,
    bench_prove_result_variants
);
criterion_main!(prover_benches);
