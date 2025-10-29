# Performance Benchmarks for raa_tt

This document describes the comprehensive benchmark suite for the `raa_tt` crate, designed to detect performance regressions and understand the performance characteristics of the major components.

## Overview

The benchmark suite covers two main components:

1. **Prover** - Tests the performance of the analytic tableaux (truth tree) method
2. **TableGenerator** - Tests the performance of truth table generation

## Prerequisites

The benchmarks use the [Criterion](https://github.com/bheisler/criterion.rs) benchmarking framework, which is included as a dev-dependency.

## Running Benchmarks

### Run All Benchmarks

```bash
cargo bench
```

### Run Specific Component Benchmarks

#### Prover Benchmarks
```bash
cargo bench --bench prover_benchmarks
```

#### Table Generator Benchmarks
```bash
cargo bench --bench table_generator_benchmarks
```

### Run Specific Benchmark Groups

#### Prover Benchmark Groups
```bash
# Simple propositions (single variables, basic operations)
cargo bench --bench prover_benchmarks -- simple_propositions

# Complex propositions (nested operations, multiple variables)
cargo bench --bench prover_benchmarks -- complex_propositions

# Known tautologies from the test suite
cargo bench --bench prover_benchmarks -- tautologies

# Worst-case scenarios (deeply nested formulas)
cargo bench --bench prover_benchmarks -- worst_case_scenarios

# Different ProveResult variants (Proven, Falsified, Contingent)
cargo bench --bench prover_benchmarks -- prove_result_variants
```

#### Table Generator Benchmark Groups
```bash
# Performance scaling with variable count (1-12 variables)
cargo bench --bench table_generator_benchmarks -- variable_scaling

# Different complexities with same variable count
cargo bench --bench table_generator_benchmarks -- complexity_scaling

# Different proposition types (conjunction, disjunction, etc.)
cargo bench --bench table_generator_benchmarks -- proposition_types

# Memory usage patterns
cargo bench --bench table_generator_benchmarks -- memory_usage
```

## Benchmark Details

### Prover Benchmarks

#### Simple Propositions
Tests basic logical operations to establish baseline performance:
- Single atoms
- Basic negation
- Simple conjunction, disjunction, implication, biimplication

#### Complex Propositions
Tests nested operations with multiple variables:
- Complex conjunction with implication: `p & (p -> q)`
- Nested implications: `(p -> q) -> (q -> r)`
- Mixed operators with 4+ variables

#### Tautologies
Tests known logical truths for validation and performance:
- Law of excluded middle: `p | !p`
- Modus ponens: `(p & (p -> q)) -> q`
- Hypothetical syllogism: `((p -> q) & (q -> r)) -> (p -> r)`

#### Worst-Case Scenarios
Tests deeply nested formulas that may trigger exponential behavior:
- Deep conjunction chains (depth 5)
- Deep disjunction chains (depth 5)
- Deep implication chains (depth 4)
- Complex nested branches

#### ProveResult Variants
Tests propositions that result in different outcomes:
- **Proven** (tautologies): Always true regardless of variable assignments
- **Falsified** (contradictions): Always false regardless of variable assignments
- **Contingent**: Truth value depends on variable assignments

### Table Generator Benchmarks

#### Variable Scaling
Tests performance as the number of variables increases (1-12 variables):
- Time complexity: O(2^n × m) where n = variables, m = proposition complexity
- Space complexity: O(2^n × n) for storing the complete truth table
- Warning threshold at 12 variables (4,096 rows)

#### Complexity Scaling
Tests different complexity levels with the same number of variables (4):
- Simple conjunction
- Mixed basic operators
- Complex nested with negations
- Deeply nested structures

#### Proposition Types
Tests different logical operators with the same variable count (3):
- Pure conjunction
- Pure disjunction
- Pure implication
- Pure biimplication
- Negation-heavy formulas

#### Memory Usage
Tests memory-intensive scenarios:
- Wide propositions (many variables, shallow depth)
- High variable count (approaching 16-variable limit)
- Maximum safe variable count (12 variables = 4,096 rows)

## Performance Expectations

### Prover Performance
- **Simple propositions**: Sub-millisecond performance
- **Complex propositions**: Millisecond to tens of milliseconds
- **Tautologies**: Fast resolution due to systematic proof method
- **Worst-case scenarios**: May exhibit exponential behavior but should complete within reasonable time

### Table Generator Performance
- **1-8 variables**: Very fast (< 1ms to ~10ms)
- **9-12 variables**: Moderate performance (10ms to ~100ms)
- **13-16 variables**: Slower but acceptable (100ms to several seconds)
- **Memory usage**: Grows exponentially with variable count

## Interpreting Results

### Criterion Output
Criterion provides detailed statistics including:
- **Mean time**: Average execution time
- **Standard deviation**: Variability in measurements
- **Throughput**: Operations per second
- **Regression detection**: Automatic detection of performance changes

### HTML Reports
Criterion generates detailed HTML reports in `target/criterion/`:
```bash
# Open the main report
open target/criterion/report/index.html
```

### Performance Regression Detection
Criterion automatically detects performance regressions by comparing against previous runs:
- **Green**: Performance improved or stable
- **Yellow**: Minor performance change
- **Red**: Significant performance regression detected

## Continuous Integration

### Running in CI
Add to your CI pipeline:
```yaml
- name: Run benchmarks
  run: cargo bench --bench prover_benchmarks --bench table_generator_benchmarks
```

### Benchmark Comparison
For regression detection in CI:
```bash
# Save baseline
cargo bench -- --save-baseline main

# Compare against baseline
cargo bench -- --baseline main
```

## Troubleshooting

### Long Running Benchmarks
If benchmarks take too long:
```bash
# Run with reduced sample size
cargo bench -- --sample-size 10

# Run specific fast benchmarks only
cargo bench --bench prover_benchmarks -- simple_propositions
```

### Memory Issues
For memory-intensive benchmarks:
```bash
# Skip heavy memory tests
cargo bench --bench table_generator_benchmarks -- variable_scaling --ignored
```

### Debug Mode
For debugging benchmark issues:
```bash
# Run in debug mode with timing
RUST_LOG=debug cargo bench --bench prover_benchmarks -- simple_propositions
```

## Extending Benchmarks

### Adding New Benchmarks
1. Add test cases to the appropriate `create_*()` function
2. Create new benchmark groups for new functionality
3. Update this documentation

### Custom Propositions
Create propositions programmatically:
```rust
use raa_tt::{proposition::Proposition, conjunction::Conjunction};

let custom_prop = Proposition::Conjunction(Conjunction {
    left: Box::new(Proposition::Atom("custom_var".to_string())),
    right: Box::new(Proposition::Atom("another_var".to_string())),
});
```

## Performance Monitoring

### Baseline Management
```bash
# Create performance baseline
cargo bench -- --save-baseline v0.8.0

# Compare current performance
cargo bench -- --baseline v0.8.0

# List available baselines
ls target/criterion/*/base/
```

### Automated Monitoring
Consider integrating with performance monitoring tools:
- [Bencher](https://bencher.dev/) for continuous benchmark tracking
- Custom CI scripts for performance regression alerts
- Integration with monitoring systems

## Best Practices

1. **Run benchmarks on dedicated hardware** for consistent results
2. **Establish baselines** before making performance-critical changes
3. **Monitor trends** over time rather than single measurements
4. **Test both fast and slow cases** to understand performance characteristics
5. **Document expected performance** for each component

## Benchmark Maintenance

- Review and update benchmarks when adding new features
- Ensure benchmark propositions cover edge cases
- Validate that benchmark results align with theoretical complexity expectations
- Update performance expectations as the codebase evolves