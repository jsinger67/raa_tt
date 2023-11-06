# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

---

## 0.4.0 - Not released yet

* The binary tool now supports direct input of expression as string.
    ```shell
    raa_tt -s "(a & a -> b) -> b" -q
    ```
* The binary tool now supports command line help by featuring `clap` command line parser.
    ```shell
    raa_tt -h
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
