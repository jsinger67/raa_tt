[![Rust](https://github.com/jsinger67/raa_tt/actions/workflows/rust.yml/badge.svg)](https://github.com/jsinger67/raa_tt/actions/workflows/rust.yml)

# raa_tt - Prover for sentences of propositional calculus

*Reductio ad absurdum* using the *truth tree* method as decision procedure for sentences of
propositional logic.

The input is parsed by a parser generated by [parol](https://github.com/jsinger67/parol) parser
generator. The grammar used for propositions can be inspected
[here](https://github.com/jsinger67/raa_tt/blob/main/raa_tt.par). `parol` transformed it into a
LL(1) grammar.

The parse result is later converted into a suitable representation [raa_tt::proposition::Proposition]
that enables symbolic processing of the given propositional expression.

Essentially, the negation of the proposition is assumed to be true, and then `raa_tt` tries to
refute that negated proposition, which indirectly proves the original proposition.

*For latest changes please see
[CHANGELOG](https://github.com/jsinger67/raa_tt/blob/main/CHANGELOG.md)*

## How to use it?

Clone this repository and switch to the repository's folder.

Then run
```shell
cargo run ./test.txt
```
or
```shell
cargo run --release  ./test.txt
```

To test your own propositions, please modify the file `test.txt`.

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
