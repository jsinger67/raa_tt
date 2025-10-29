//! Benchmarks for the TableGenerator component
//!
//! This benchmark suite tests the performance of truth table generation across different
//! numbers of variables and proposition complexities to detect performance regressions
//! and understand scaling characteristics.
//!
//! ## Running the benchmarks
//!
//! To run all table generator benchmarks:
//! ```bash
//! cargo bench --bench table_generator_benchmarks
//! ```
//!
//! To run specific benchmark groups:
//! ```bash
//! cargo bench --bench table_generator_benchmarks -- variable_scaling
//! cargo bench --bench table_generator_benchmarks -- complexity_scaling
//! cargo bench --bench table_generator_benchmarks -- proposition_types
//! cargo bench --bench table_generator_benchmarks -- memory_usage
//! ```

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use raa_tt::{
    bi_implication::BiImplication, conjunction::Conjunction, disjunction::Disjunction,
    implication::Implication, negation::Negation, proposition::Proposition,
    table_generator::TableGenerator,
};

/// Generate propositions with different numbers of variables for scaling tests
fn create_propositions_by_variable_count() -> Vec<(usize, Proposition)> {
    (1..=12)
        .map(|count| (count, create_conjunction_chain(count)))
        .collect()
}

/// Create a conjunction chain with exactly `count` variables
/// Creates: var0 & var1 & var2 & ... & var(count-1)
fn create_conjunction_chain(count: usize) -> Proposition {
    if count == 0 {
        return Proposition::Atom("var0".to_string());
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

/// Create a disjunction chain with exactly `count` variables
/// Creates: var0 | var1 | var2 | ... | var(count-1)
fn create_disjunction_chain(count: usize) -> Proposition {
    if count == 0 {
        return Proposition::Atom("var0".to_string());
    }

    let mut proposition = Proposition::Atom("var0".to_string());

    for i in 1..count {
        let next_var = Proposition::Atom(format!("var{}", i));
        proposition = Proposition::Disjunction(Disjunction {
            left: Box::new(proposition),
            right: Box::new(next_var),
        });
    }

    proposition
}

/// Create an implication chain with exactly `count` variables
/// Creates: var0 -> (var1 -> (var2 -> ... -> var(count-1)))
fn create_implication_chain(count: usize) -> Proposition {
    if count == 0 {
        return Proposition::Atom("var0".to_string());
    }
    if count == 1 {
        return Proposition::Atom("var0".to_string());
    }

    let mut proposition = Proposition::Atom(format!("var{}", count - 1));

    for i in (0..count - 1).rev() {
        let current_var = Proposition::Atom(format!("var{}", i));
        proposition = Proposition::Implication(Implication {
            left: Box::new(current_var),
            right: Box::new(proposition),
        });
    }

    proposition
}

/// Create different complexity levels with the same number of variables (4)
fn create_complexity_variations() -> Vec<(&'static str, Proposition)> {
    vec![
        // Level 1: Simple conjunction
        (
            "simple_conjunction_4vars",
            Proposition::Conjunction(Conjunction {
                left: Box::new(Proposition::Conjunction(Conjunction {
                    left: Box::new(Proposition::Atom("a".to_string())),
                    right: Box::new(Proposition::Atom("b".to_string())),
                })),
                right: Box::new(Proposition::Conjunction(Conjunction {
                    left: Box::new(Proposition::Atom("c".to_string())),
                    right: Box::new(Proposition::Atom("d".to_string())),
                })),
            }),
        ),
        // Level 2: Mixed basic operators
        (
            "mixed_basic_4vars",
            Proposition::Conjunction(Conjunction {
                left: Box::new(Proposition::Disjunction(Disjunction {
                    left: Box::new(Proposition::Atom("a".to_string())),
                    right: Box::new(Proposition::Atom("b".to_string())),
                })),
                right: Box::new(Proposition::Implication(Implication {
                    left: Box::new(Proposition::Atom("c".to_string())),
                    right: Box::new(Proposition::Atom("d".to_string())),
                })),
            }),
        ),
        // Level 3: Complex nested with negations
        (
            "complex_nested_4vars",
            Proposition::BiImplication(BiImplication {
                left: Box::new(Proposition::Conjunction(Conjunction {
                    left: Box::new(Proposition::Negation(Negation {
                        inner: Box::new(Proposition::Atom("a".to_string())),
                    })),
                    right: Box::new(Proposition::Atom("b".to_string())),
                })),
                right: Box::new(Proposition::Disjunction(Disjunction {
                    left: Box::new(Proposition::Atom("c".to_string())),
                    right: Box::new(Proposition::Negation(Negation {
                        inner: Box::new(Proposition::Atom("d".to_string())),
                    })),
                })),
            }),
        ),
        // Level 4: Deeply nested
        (
            "deeply_nested_4vars",
            Proposition::Implication(Implication {
                left: Box::new(Proposition::Conjunction(Conjunction {
                    left: Box::new(Proposition::Implication(Implication {
                        left: Box::new(Proposition::Atom("a".to_string())),
                        right: Box::new(Proposition::Negation(Negation {
                            inner: Box::new(Proposition::Atom("b".to_string())),
                        })),
                    })),
                    right: Box::new(Proposition::BiImplication(BiImplication {
                        left: Box::new(Proposition::Atom("c".to_string())),
                        right: Box::new(Proposition::Atom("d".to_string())),
                    })),
                })),
                right: Box::new(Proposition::Disjunction(Disjunction {
                    left: Box::new(Proposition::Negation(Negation {
                        inner: Box::new(Proposition::Atom("a".to_string())),
                    })),
                    right: Box::new(Proposition::Negation(Negation {
                        inner: Box::new(Proposition::Atom("c".to_string())),
                    })),
                })),
            }),
        ),
    ]
}

/// Create propositions with different operator types but same variable count
fn create_proposition_types() -> Vec<(&'static str, Proposition)> {
    vec![
        // Pure conjunction (3 variables)
        (
            "pure_conjunction_3vars",
            Proposition::Conjunction(Conjunction {
                left: Box::new(Proposition::Conjunction(Conjunction {
                    left: Box::new(Proposition::Atom("p".to_string())),
                    right: Box::new(Proposition::Atom("q".to_string())),
                })),
                right: Box::new(Proposition::Atom("r".to_string())),
            }),
        ),
        // Pure disjunction (3 variables)
        (
            "pure_disjunction_3vars",
            Proposition::Disjunction(Disjunction {
                left: Box::new(Proposition::Disjunction(Disjunction {
                    left: Box::new(Proposition::Atom("p".to_string())),
                    right: Box::new(Proposition::Atom("q".to_string())),
                })),
                right: Box::new(Proposition::Atom("r".to_string())),
            }),
        ),
        // Pure implication (3 variables)
        (
            "pure_implication_3vars",
            Proposition::Implication(Implication {
                left: Box::new(Proposition::Atom("p".to_string())),
                right: Box::new(Proposition::Implication(Implication {
                    left: Box::new(Proposition::Atom("q".to_string())),
                    right: Box::new(Proposition::Atom("r".to_string())),
                })),
            }),
        ),
        // Pure biimplication (3 variables)
        (
            "pure_biimplication_3vars",
            Proposition::BiImplication(BiImplication {
                left: Box::new(Proposition::Atom("p".to_string())),
                right: Box::new(Proposition::BiImplication(BiImplication {
                    left: Box::new(Proposition::Atom("q".to_string())),
                    right: Box::new(Proposition::Atom("r".to_string())),
                })),
            }),
        ),
        // Negation heavy (3 variables)
        (
            "negation_heavy_3vars",
            Proposition::Conjunction(Conjunction {
                left: Box::new(Proposition::Negation(Negation {
                    inner: Box::new(Proposition::Negation(Negation {
                        inner: Box::new(Proposition::Atom("p".to_string())),
                    })),
                })),
                right: Box::new(Proposition::Disjunction(Disjunction {
                    left: Box::new(Proposition::Negation(Negation {
                        inner: Box::new(Proposition::Atom("q".to_string())),
                    })),
                    right: Box::new(Proposition::Negation(Negation {
                        inner: Box::new(Proposition::Atom("r".to_string())),
                    })),
                })),
            }),
        ),
    ]
}

/// Memory-intensive propositions for testing memory usage patterns
fn create_memory_test_propositions() -> Vec<(&'static str, Proposition)> {
    vec![
        // Wide proposition (many variables, shallow)
        ("wide_8vars", create_conjunction_chain(8)),
        // Medium width, medium depth
        ("medium_6vars", create_complexity_with_variables(6)),
        // High variable count (approaching limit)
        ("high_10vars", create_conjunction_chain(10)),
        // Maximum safe variable count
        ("max_safe_12vars", create_conjunction_chain(12)),
    ]
}

/// Create a moderately complex proposition with specified number of variables
fn create_complexity_with_variables(var_count: usize) -> Proposition {
    if var_count < 2 {
        return Proposition::Atom("var0".to_string());
    }

    let half = var_count / 2;
    let left_vars = create_conjunction_chain(half);
    let right_vars = create_disjunction_chain(var_count - half);

    Proposition::Implication(Implication {
        left: Box::new(left_vars),
        right: Box::new(right_vars),
    })
}

/// Benchmark truth table generation performance scaling with variable count
fn bench_variable_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_generator_variable_scaling");

    for (var_count, proposition) in create_propositions_by_variable_count() {
        group.bench_with_input(
            BenchmarkId::new("generate_table", format!("{}_vars", var_count)),
            &proposition,
            |b, prop| {
                b.iter(|| {
                    let generator = TableGenerator::new();
                    generator.generate_truth_table(prop).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark truth table generation with different complexity levels
fn bench_complexity_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_generator_complexity_scaling");

    for (name, proposition) in create_complexity_variations() {
        group.bench_with_input(
            BenchmarkId::new("generate_table", name),
            &proposition,
            |b, prop| {
                b.iter(|| {
                    let generator = TableGenerator::new();
                    generator.generate_truth_table(prop).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark different proposition types
fn bench_proposition_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_generator_proposition_types");

    for (name, proposition) in create_proposition_types() {
        group.bench_with_input(
            BenchmarkId::new("generate_table", name),
            &proposition,
            |b, prop| {
                b.iter(|| {
                    let generator = TableGenerator::new();
                    generator.generate_truth_table(prop).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark memory-intensive scenarios
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_generator_memory_usage");

    for (name, proposition) in create_memory_test_propositions() {
        group.bench_with_input(
            BenchmarkId::new("generate_table", name),
            &proposition,
            |b, prop| {
                b.iter(|| {
                    let generator = TableGenerator::new();
                    generator.generate_truth_table(prop).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark table generator creation overhead
fn bench_generator_creation(c: &mut Criterion) {
    c.bench_function("table_generator_creation", |b| {
        b.iter(|| TableGenerator::new());
    });
}

/// Benchmark variable extraction performance (indirectly through TableGenerator)
fn bench_variable_extraction(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_generator_variable_extraction");

    let test_cases = vec![
        ("simple_2vars", create_conjunction_chain(2)),
        ("medium_5vars", create_conjunction_chain(5)),
        ("complex_4vars", create_complexity_variations()[2].1.clone()),
        ("high_8vars", create_conjunction_chain(8)),
    ];

    for (name, proposition) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("extract_variables_via_generator", name),
            &proposition,
            |b, prop| {
                b.iter(|| {
                    // We can indirectly measure variable extraction by creating a TableGenerator
                    // and starting the truth table generation process
                    let generator = TableGenerator::new();
                    // The variable extraction happens early in generate_truth_table
                    // We'll measure the full process but focus on smaller propositions
                    generator.generate_truth_table(prop).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark truth value calculation performance
fn bench_truth_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_generator_truth_calculation");

    let test_cases = vec![
        ("simple_conjunction", create_conjunction_chain(3)),
        ("simple_disjunction", create_disjunction_chain(3)),
        ("complex_mixed", create_complexity_variations()[2].1.clone()),
    ];

    for (name, proposition) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("calculate_value_via_table", name),
            &proposition,
            |b, prop| {
                // We measure truth calculation by generating the full truth table
                // This includes both variable extraction and value calculation
                b.iter(|| {
                    let generator = TableGenerator::new();
                    generator.generate_truth_table(prop).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark performance at warning threshold
fn bench_warning_threshold(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_generator_warning_threshold");

    // Test exactly at and around the warning threshold (12 variables)
    let test_cases = vec![
        ("just_under_warning_11vars", create_conjunction_chain(11)),
        ("at_warning_12vars", create_conjunction_chain(12)),
    ];

    for (name, proposition) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("generate_table", name),
            &proposition,
            |b, prop| {
                b.iter(|| {
                    let generator = TableGenerator::new();
                    generator.generate_truth_table(prop).unwrap()
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    table_generator_benches,
    bench_generator_creation,
    bench_variable_extraction,
    bench_truth_calculation,
    bench_variable_scaling,
    bench_complexity_scaling,
    bench_proposition_types,
    bench_memory_usage,
    bench_warning_threshold
);
criterion_main!(table_generator_benches);
