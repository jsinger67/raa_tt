# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

---

## 0.9.1 - 2025-12-18

* Improve error reporting when working with direct input text (-s)
* Update dependencies

## 0.9.0 - 2025-11-22

* As an example on how to use another error reporting crate than `codespan-reporting` which is the
default in `parol_runtime`, the binary tool now features a custom error reporter based on `ariadne`.

Use `cargo run --bin raa_tt -- -f error.txt` resp. `raa_tt -f error.txt` to see the error messages.
I also turned on the error recovery of the parser again. This way multiple errors can be reported.

## 0.8.0 - 2025-10-29

* Upgrade to parol 4.1
* Upgrade other dependencies
* Enhance documentation of public APIs
* Truth table generation
    * Improve error handling in truth table generation when the number of variable exceed the limit
    * Add variable count tests for truth table generation
    * Improved formatting of truth tables
* Switch to edition 2024
* Improved README

## 0.7.0 - 2025-03-14

* Upgrade parol and parol_runtime to versions 3.0.1 and 3.0.0, respectively

## 0.6.0 - 2024-12-28

* Upgrade to parol 2.0

## 0.5.0 - 2024-09-19

* Upgrade to parol 1.0
* Fix new clippy warning

## 0.4.0 - 2023-11-09

* The binary tool now supports direct input of expression as string.
    ```shell
    raa_tt -s "(a & a -> b) -> b" -q
    ```
* The binary tool now supports command line help by featuring `clap` command line parser.
    ```shell
    raa_tt -h
    ```
* The binary tool can now additionally generate truth tables. This is only for reference reasons
since many available tools understand or generate truth tables. The limit for the number of
variables for which a truth table can be generated is currently deliberately set to 16.
    ```shell
    raa_tt -s "(a & a -> b) -> b" -q -t
    ```

## 0.3.0 - 2023-11-05

* Changed name of binary tool to `raa_tt`.
* Hopefully fixed problem running `cargo doc` on doc.rs's readonly file system

## 0.2.0 - 2023-11-05

* Grammar supports an arbitrary number of propositions.
* Add more examples to `test.txt`
* Add integration tests
* Improved naming and better English - hopefully 😉
* Add generated files to source control to prevent warnings during packaging

## 0.1.0 - 2023-11-01

* Basic algorithm
* Grammar supports only one proposition at once.
