# raa_tt - Prover for sentences of propositional calculus

*Reductio ad absurdum* using the *truth tree* method as decision procedure for sentences of
propositional logic.

The parsed input (using [parol](https://github.com/jsinger67/parol) as parser generator) is
converted in an internal form that is needed to symbolically process a propositional expression.

Essentially, the negation of the proposition is assumed to be true, and then the tool tries to
refute that negated proposition, which indirectly proves the original proposition.

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

### You want to debug the algorithm?
To support this you can use the logging feature.

Essentially set the `RUST_LOG` environment variable like in these example which use the powershell:

```powershell
$env:RUST_LOG="raa_tt=trace,raa_tt::raa_tt_grammar_trait=error"
```
or
```powershell
$env:RUST_LOG="raa_tt=debug,raa_tt::raa_tt_grammar_trait=error"
```
