[![Rust](https://github.com/jsinger67/raa_tt/actions/workflows/rust.yml/badge.svg)](https://github.com/jsinger67/raa_tt/actions/workflows/rust.yml)
[![Docs.rs](https://docs.rs/raa_tt/badge.svg)](https://docs.rs/raa_tt)
[![Crates.io](https://img.shields.io/crates/v/raa_tt.svg)](https://crates.io/crates/raa_tt)

# raa_tt - Prover for sentences of propositional calculus

This crate provides an algorithm to decide whether a propositional formula is a tautology, a
contradiction or contingent, i.e. its truth value depends on the truth values of its variables.

The algorithm used here is a form of *Reductio ad absurdum*, namely the
[truth tree](https://en.wikipedia.org/wiki/Method_of_analytic_tableaux) method, a decision procedure
for sentences of propositional logic. Especially for a larger number of logical variables this
method is more efficient than other methods like e.g.
[truth tables](https://en.wikipedia.org/wiki/Truth_table)

For easy use, the formula can be provided in textual form and is parsed by `raa_tt` binary tool.
The grammar used for propositions can be inspected
[here](https://github.com/jsinger67/raa_tt/blob/main/raa_tt.par).

*For latest changes please see
[CHANGELOG](https://github.com/jsinger67/raa_tt/blob/main/CHANGELOG.md)*

## How to use it?

Clone this repository and switch to the repository's folder.

Then run, e.g.
```shell
cargo run -- -f ./test.txt
cargo run --release -- -s "((p -> q) & (r -> s) & (p | r)) -> q | s"
```

Alternatively you can install `raa_tt` via
```shell
cargo install raa_tt
```
Then you can call it directly from the command line, e.g. like

```shell
raa_tt -s "(a & a -> b) -> b" -q
```

You can use command line argument `-h` to get help.


## Can you embed it into your own application?

Yes! `raa_tt` is a library, too. You can reference it in your own crate.

## You want to explore the algorithm?
To support this you can use the logging feature.

Essentially set the `RUST_LOG` environment variable like in these examples which use the powershell:

```powershell
$env:RUST_LOG="raa_tt=trace,raa_tt::raa_tt_grammar_trait=error"
```
or
```powershell
$env:RUST_LOG="raa_tt=debug,raa_tt::raa_tt_grammar_trait=error"
```
## You found a bug?

Please, file an issue.
