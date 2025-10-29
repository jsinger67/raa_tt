# raa_tt - Propositional Logic Prover

[![Rust](https://github.com/jsinger67/raa_tt/actions/workflows/rust.yml/badge.svg)](https://github.com/jsinger67/raa_tt/actions/workflows/rust.yml)
[![Docs.rs](https://docs.rs/raa_tt/badge.svg)](https://docs.rs/raa_tt)
[![Crates.io](https://img.shields.io/crates/v/raa_tt.svg)](https://crates.io/crates/raa_tt)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A sophisticated Rust library and CLI tool for proving sentences of propositional calculus using the **analytic tableaux method** (truth trees). This implementation provides both a high-performance library API and an intuitive command-line interface for logical reasoning and educational purposes.

## Table of Contents

- [Features](#features)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Grammar Reference](#grammar-reference)
- [Usage Examples](#usage-examples)
  - [Command Line Interface](#command-line-interface)
  - [Library Usage](#library-usage)
- [Mathematical Background](#mathematical-background)
- [Algorithm & Architecture](#algorithm--architecture)
- [Performance](#performance)
- [API Documentation](#api-documentation)
- [Development](#development)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## Features

🔬 **Advanced Proof Engine**
- Analytic tableaux (truth tree) method for logical proving
- Systematic detection of tautologies, contradictions, and contingent formulas
- Efficient branch pruning and contradiction detection

📊 **Truth Table Generation**
- Complete truth table computation for verification
- Configurable variable limits (up to 16 variables)
- Memory-efficient implementation with safety checks

🎯 **Dual Interface**
- **Library**: Programmatic API for integration into Rust applications
- **CLI Tool**: User-friendly command-line interface with rich output

⚡ **High Performance**
- Optimized for complex logical formulas
- Comprehensive benchmark suite included
- Early contradiction detection for faster proving

🧮 **Rich Grammar Support**
- Complete propositional logic syntax
- Operator precedence following mathematical conventions
- Support for all standard logical connectives

🔍 **Developer-Friendly**
- Extensive documentation with examples
- Detailed error handling and reporting
- Debug logging for algorithm exploration

## Quick Start

### CLI Usage
```shell
# Install the tool
cargo install raa_tt

# Prove a simple tautology
raa_tt -s "p | !p"

# Analyze a complex formula with truth table
raa_tt -s "((p -> q) & (r -> s) & (p | r)) -> (q | s)" -t

# Process formulas from a file
raa_tt -f formula.txt -q
```

### Library Usage
```rust
use raa_tt::{
    prover::{Prover, ProveResult},
    proposition::Proposition,
    raa_tt_parser::parse,
    raa_tt_grammar::RaaTtGrammar,
};

// Parse and prove a formula
let mut grammar = RaaTtGrammar::new();
parse("p -> (q -> p)", &"example".into(), &mut grammar)?;

let proposition: Proposition = (&grammar.raa_tt.unwrap().raa_tt_list[0].biconditional).into();
let prover = Prover::new();
let result = prover.prove(&proposition)?;

match result {
    ProveResult::Proven => println!("Tautology: Always true"),
    ProveResult::Falsified => println!("Contradiction: Always false"),
    ProveResult::Contingent => println!("Contingent: Depends on variables"),
    _ => unreachable!(),
}
```

## Installation

### As a CLI Tool

**From crates.io (Recommended):**
```shell
cargo install raa_tt
```

**From source:**
```shell
git clone https://github.com/jsinger67/raa_tt.git
cd raa_tt
cargo install --path .
```

### As a Library Dependency

Add to your `Cargo.toml`:
```toml
[dependencies]
raa_tt = "0.8.0"
```

### Development Setup

```shell
# Clone repository
git clone https://github.com/jsinger67/raa_tt.git
cd raa_tt

# Run tests
cargo test

# Run benchmarks
cargo bench

# Build documentation
cargo doc --open
```

## Grammar Reference

The `raa_tt` parser supports a rich grammar for propositional logic expressions with standard operator precedence.

### Logical Operators

| Operator | Symbol | Description | Precedence | Example |
|----------|---------|-------------|------------|----------|
| Negation | `!` | NOT | 1 (highest) | `!p` |
| Conjunction | `&` | AND | 2 | `p & q` |
| Disjunction | `\|` | OR | 3 | `p \| q` |
| Implication | `->` | IF...THEN | 4 | `p -> q` |
| Biimplication | `<->` | IF AND ONLY IF | 5 (lowest) | `p <-> q` |

### Variables and Syntax

- **Variables**: Start with lowercase letter, followed by letters, digits, or underscores
  - Valid: `p`, `q1`, `var_name`, `proposition_a`
  - Invalid: `P`, `1var`, `-name`

- **Parentheses**: Use `()` for grouping and overriding precedence
  - `(p | q) & r` vs `p | (q & r)`

- **Comments**: Line comments start with `//`

### Grammar Examples

**Basic Operations:**
```
p                    // Simple atom
!p                   // Negation
p & q                // Conjunction
p | q                // Disjunction
p -> q               // Implication
p <-> q              // Biimplication
```

**Complex Expressions:**
```
// Modus Ponens
(p & (p -> q)) -> q

// Law of Excluded Middle
p | !p

// De Morgan's Law
!(p & q) <-> (!p | !q)

// Complex nested formula
((p -> q) & (q -> r)) -> (p -> r)
```

**Precedence Examples:**
```
!p & q               // Equivalent to (!p) & q
p | q & r            // Equivalent to p | (q & r)
p -> q | r           // Equivalent to p -> (q | r)
p & q -> r           // Equivalent to (p & q) -> r
p <-> q -> r         // Equivalent to p <-> (q -> r)
```

## Usage Examples

### Command Line Interface

#### Basic Proving
```shell
# Test a tautology
raa_tt -s "p -> p"
# Output: p -> p is Logically True

# Test a contradiction
raa_tt -s "p & !p"
# Output: p & !p is Logically False

# Test a contingent formula
raa_tt -s "p & q"
# Output: p & q is Contingent
```

#### Truth Table Generation
```shell
# Generate truth table for a formula
raa_tt -s "p -> q" -t

# Output includes:
# (p -> q) is Contingent
# p | q | (p -> q) |
# -------------------
# F | F |        T |
# F | T |        T |
# T | F |        F |
# T | T |        T |
```

#### File Input
```shell
# Create a file with formulas
echo "p | !p" > formulas.txt
echo "(p -> q) & p -> q" >> formulas.txt

# Process the file
raa_tt -f formulas.txt

# Quiet mode (minimal output)
raa_tt -f formulas.txt -q
```

#### Advanced Usage
```shell
# Complex formula with debugging
RUST_LOG=raa_tt=debug raa_tt -s "((p -> q) & (q -> r)) -> (p -> r)"

# Batch processing with truth tables
raa_tt -f complex_formulas.txt -t > results.txt
```

### Library Usage

#### Basic Proving
```rust
use raa_tt::{
    prover::{Prover, ProveResult},
    proposition::Proposition,
    conjunction::Conjunction,
    implication::Implication,
    negation::Negation,
};

// Create propositions programmatically
let modus_ponens = Proposition::Implication(Implication {
    left: Box::new(Proposition::Conjunction(Conjunction {
        left: Box::new("p".into()),
        right: Box::new(Proposition::Implication(Implication {
            left: Box::new("p".into()),
            right: Box::new("q".into()),
        })),
    })),
    right: Box::new("q".into()),
});

let prover = Prover::new();
let result = prover.prove(&modus_ponens)?;
assert_eq!(result, ProveResult::Proven);
```

#### Parsing Text Formulas
```rust
use raa_tt::{
    raa_tt_parser::parse,
    raa_tt_grammar::RaaTtGrammar,
    prover::Prover,
};

fn prove_formula(formula: &str) -> Result<ProveResult, Box<dyn std::error::Error>> {
    let mut grammar = RaaTtGrammar::new();
    parse(formula, &"input".into(), &mut grammar)?;
    
    let proposition = (&grammar.raa_tt.unwrap().raa_tt_list[0].biconditional).into();
    let prover = Prover::new();
    Ok(prover.prove(&proposition)?)
}

// Usage
let result = prove_formula("(p -> q) & p -> q")?;
println!("Formula is: {}", result);
```

#### Truth Table Generation
```rust
use raa_tt::{
    table_generator::TableGenerator,
    proposition::Proposition,
    conjunction::Conjunction,
};

let proposition = Proposition::Conjunction(Conjunction {
    left: Box::new("p".into()),
    right: Box::new("q".into()),
});

let generator = TableGenerator::new();
let truth_table = generator.generate_truth_table(&proposition)?;
println!("{}", truth_table);
```

#### Error Handling
```rust
use raa_tt::errors::{RaaError, Result};

fn safe_prove(formula: &str) -> Result<ProveResult> {
    // ... parsing and proving logic
    match result {
        Err(RaaError::TooManyVariables { current, max, .. }) => {
            eprintln!("Too many variables: {} (max: {})", current, max);
            Err(result.unwrap_err())
        },
        Err(RaaError::UndefinedVariable { name }) => {
            eprintln!("Undefined variable: {}", name);
            Err(result.unwrap_err())
        },
        _ => result,
    }
}
```

## Mathematical Background

### Propositional Logic

Propositional logic is a branch of mathematical logic that deals with propositions and their relationships through logical connectives. A proposition is a declarative statement that is either true or false.

**Key Concepts:**
- **Tautology**: A formula that is always true regardless of variable assignments
- **Contradiction**: A formula that is always false regardless of variable assignments  
- **Contingent**: A formula whose truth value depends on the specific variable assignments

### Truth Trees (Analytic Tableaux)

The truth tree method is a decision procedure for propositional logic that systematically explores logical relationships by:

1. **Assumption**: Starting with the negation of the formula to be proved
2. **Decomposition**: Breaking complex formulas into simpler components
3. **Branching**: Creating separate paths for disjunctive cases
4. **Closure**: Identifying contradictions (P ∧ ¬P) to close branches
5. **Resolution**: Determining the formula's logical status

**Transformation Rules:**

| Formula Type | Decomposition |
|--------------|---------------|
| A ∧ B | Add A and B to current branch |
| A ∨ B | Create two branches: one with A, one with B |
| A → B | Create two branches: one with ¬A, one with B |
| A ↔ B | Create two branches: (A∧B) and (¬A∧¬B) |
| ¬(A ∧ B) | Add ¬A and ¬B to current branch |
| ¬(A ∨ B) | Create two branches: one with ¬A, one with ¬B |
| ¬¬A | Add A to current branch |

## Algorithm & Architecture

### Core Components

The `raa_tt` library is built around several key components:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Parser        │    │   Prover        │    │ TableGenerator  │
│                 │    │                 │    │                 │
│ • Grammar       │───▶│ • Truth Trees   │    │ • Truth Tables  │
│ • Tokenization  │    │ • Branch Logic  │    │ • Enumeration   │
│ • AST Building  │    │ • Contradiction │    │ • Formatting    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Proposition    │    │  ProveResult    │    │  TruthTable     │
│                 │    │                 │    │                 │
│ • Atoms         │    │ • Proven        │    │ • Variables     │
│ • Operators     │    │ • Falsified     │    │ • Rows          │
│ • Expressions   │    │ • Contingent    │    │ • Evaluation    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Complexity Analysis

**Time Complexity:**
- **Prover**: O(2^n) worst case, where n is the number of variables
  - Many formulas resolve faster due to early contradiction detection
  - Tautologies often prove quickly through systematic closure
  
- **Truth Table**: O(2^n × m) where m is formula evaluation complexity
  - Guaranteed exponential growth with variable count
  - 16-variable limit prevents excessive memory usage

**Space Complexity:**
- **Prover**: O(2^n) for tree structure in worst case
- **Truth Table**: O(2^n × n) for complete table storage

### Error Handling

The library provides comprehensive error handling through the [`RaaError`](src/errors.rs) enum:

```rust
pub enum RaaError {
    VoidExpression,                    // Internal logic error
    TooManyVariables { ... },          // Variable limit exceeded
    UndefinedVariable { name },        // Reference to undefined variable
    FormatError { source },            // Display/formatting error
}
```

## Performance

### Benchmark Suite

`raa_tt` includes a comprehensive benchmark suite to monitor performance and detect regressions. See [BENCHMARKS.md](BENCHMARKS.md) for detailed information.

**Run benchmarks:**
```shell
# All benchmarks
cargo bench

# Specific components
cargo bench --bench prover_benchmarks
cargo bench --bench table_generator_benchmarks
```

### Performance Characteristics

**Prover Performance:**
- Simple propositions: Sub-millisecond
- Complex nested formulas: Milliseconds to seconds
- Worst-case scenarios: May exhibit exponential behavior

**Truth Table Performance:**
- 1-8 variables: Very fast (< 1ms to ~10ms)
- 9-12 variables: Moderate (10ms to ~100ms)  
- 13-16 variables: Slower but acceptable (100ms to several seconds)

**Memory Limits:**
- Truth tables limited to 16 variables (65,536 rows)
- Automatic memory estimation and warnings
- Graceful error handling for exceeded limits

### Optimization Tips

1. **Use the prover for large formulas**: Truth trees often faster than truth tables
2. **Simplify complex expressions**: Break down into smaller sub-formulas
3. **Leverage operator precedence**: Reduce parentheses for cleaner expressions
4. **Enable quiet mode**: Use `-q` flag for batch processing

## API Documentation

### Core Types

**[`Prover`](src/prover.rs)**: Main proving engine
```rust
impl Prover {
    pub fn new() -> Self                                    // Create new prover
    pub fn prove(&self, proposition: &Proposition) -> Result<ProveResult>  // Prove formula
}
```

**[`ProveResult`](src/prover.rs)**: Proving outcomes
```rust
pub enum ProveResult {
    Proven,      // Tautology (always true)
    Falsified,   // Contradiction (always false)  
    Contingent,  // Depends on variable assignments
}
```

**[`Proposition`](src/proposition.rs)**: Logical expressions
```rust
pub enum Proposition {
    Atom(String),                              // Propositional variable
    Negation(Negation),                        // ¬P
    Conjunction(Conjunction),                  // P ∧ Q
    Disjunction(Disjunction),                  // P ∨ Q
    Implication(Implication),                  // P → Q
    BiImplication(BiImplication),              // P ↔ Q
    Void,                                      // Internal use
}
```

**[`TableGenerator`](src/table_generator.rs)**: Truth table creation
```rust
impl TableGenerator {
    pub fn new() -> Self                                    // Create generator
    pub fn generate_truth_table(&self, proposition: &Proposition) -> Result<TruthTable>
}
```

### Parser API

```rust
use raa_tt::{raa_tt_parser::parse, raa_tt_grammar::RaaTtGrammar};

let mut grammar = RaaTtGrammar::new();
parse(input, &file_name, &mut grammar)?;
let propositions = grammar.raa_tt.unwrap().raa_tt_list;
```

For complete API documentation, run:
```shell
cargo doc --open
```

## Development

### Project Structure

```
raa_tt/
├── src/
│   ├── lib.rs                 # Library root
│   ├── prover.rs             # Truth tree prover
│   ├── proposition.rs        # Logical expressions
│   ├── table_generator.rs    # Truth table generator
│   ├── truth_table.rs        # Truth table representation
│   ├── errors.rs             # Error types
│   ├── raa_tt_parser.rs      # Generated parser
│   ├── raa_tt_grammar.rs     # Grammar implementation
│   ├── conjunction.rs        # AND operator
│   ├── disjunction.rs        # OR operator
│   ├── implication.rs        # IMPLIES operator
│   ├── bi_implication.rs     # IFF operator
│   ├── negation.rs           # NOT operator
│   └── bin/
│       └── raa_tt/
│           ├── main.rs       # CLI entry point
│           └── arguments.rs  # CLI argument parsing
├── tests/
│   └── integration_tests.rs  # Integration tests
├── benches/
│   ├── prover_benchmarks.rs  # Prover performance tests
│   └── table_generator_benchmarks.rs  # Table performance tests
├── raa_tt.par               # Grammar definition
├── build.rs                 # Build script
└── README.md                # This file
```

### Building from Source

```shell
# Clone repository
git clone https://github.com/jsinger67/raa_tt.git
cd raa_tt

# Build library
cargo build --release

# Build CLI tool
cargo build --release --bin raa_tt

# Install locally
cargo install --path .
```

### Running Tests

```shell
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# Documentation tests
cargo test --doc

# All tests with output
cargo test -- --nocapture
```

### Grammar Development

The grammar is defined in [`raa_tt.par`](raa_tt.par) and processed by the [Parol](https://github.com/jsinger67/parol) parser generator:

```shell
# Regenerate parser (if modifying grammar)
parol -f ./raa_tt.par -e ./raa_tt-exp.par -p ./src/raa_tt_parser.rs \
      -a ./src/raa_tt_grammar_trait.rs -t RaaTtGrammar -m raa_tt_grammar \
      --trim --disable-recovery --minbox
```

### Debugging and Logging

Enable detailed logging to explore the algorithm:

**PowerShell:**
```powershell
$env:RUST_LOG="raa_tt=trace,raa_tt::raa_tt_grammar_trait=error"
raa_tt -s "your_formula_here"
```

**Bash/Zsh:**
```bash
RUST_LOG="raa_tt=debug" raa_tt -s "your_formula_here"
```

**Log Levels:**
- `error`: Critical errors only
- `warn`: Warnings and errors  
- `info`: General information
- `debug`: Detailed debugging info
- `trace`: Exhaustive algorithm traces

## Troubleshooting

### Common Issues

**"Too many variables for truth table generation"**
```
Error: Too many variables for truth table generation.
Current: 17 variables, Maximum allowed: 16 variables.
```
*Solution*: Use the prover instead of truth tables for complex formulas, or simplify the expression.

**"Variable X not defined"**
```
Error: Variable X not defined
```
*Solution*: Ensure all variables use lowercase letters and valid naming conventions.

**Parser errors**
```
Error occurred: Parse error at line 1, column 5
```
*Solution*: Check grammar syntax, operator precedence, and parentheses matching.

### Performance Issues

**Slow proving for complex formulas:**
- Break down into smaller sub-expressions
- Use logical equivalences to simplify
- Check for deeply nested operators

**Memory issues with truth tables:**
- Reduce number of variables (< 13 recommended for interactive use)
- Use prover for logical validation instead of truth tables
- Consider SAT solvers for very large problems

### Getting Help

1. **Check the documentation**: `cargo doc --open`
2. **Review examples**: See [`tests/`](tests/) directory
3. **Enable debug logging**: Use `RUST_LOG=debug`
4. **File an issue**: [GitHub Issues](https://github.com/jsinger67/raa_tt/issues)

## Contributing

We welcome contributions! Here's how to get started:

### Development Setup

1. **Fork and clone** the repository
2. **Set up development environment**:
   ```shell
   git clone https://github.com/your-username/raa_tt.git
   cd raa_tt
   cargo build
   cargo test
   ```

3. **Install development tools**:
   ```shell
   rustup component add rustfmt clippy
   cargo install cargo-tarpaulin  # For coverage
   ```

### Contribution Guidelines

**Code Style:**
- Run `cargo fmt` before committing
- Fix all `cargo clippy` warnings
- Add tests for new functionality
- Update documentation for public APIs

**Pull Request Process:**
1. Create a feature branch: `git checkout -b feature/your-feature`
2. Make your changes with tests
3. Run the full test suite: `cargo test`
4. Run benchmarks if affecting performance: `cargo bench`
5. Update documentation if needed
6. Submit a pull request with clear description

**Areas for Contribution:**
- Performance optimizations
- Additional logical operators
- Enhanced error messages
- Documentation improvements
- Example programs
- Educational materials

### Code of Conduct

Please be respectful and constructive in all interactions. We're committed to providing a welcoming environment for all contributors.

## License

This project is licensed under either of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

**Changelog**: See [CHANGELOG.md](CHANGELOG.md) for version history and release notes.

**Performance Benchmarks**: Detailed benchmark information available in [BENCHMARKS.md](BENCHMARKS.md).
