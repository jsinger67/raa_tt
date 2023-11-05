use std::{fs, path::PathBuf, process};

use parol::{build::Builder, ParolErrorReporter};
use parol_runtime::Report;

fn main() {
    let par_file = PathBuf::from("raa_tt.par");
    let exp_file = PathBuf::from("raa_tt-exp.par");

    let par_modified = fs::metadata(par_file).unwrap().modified().unwrap();
    let exp_modified = fs::metadata(exp_file).unwrap().modified().unwrap();

    if par_modified <= exp_modified {
        return;
    }

    // CLI equivalent is:
    // parol -f ./raa_tt.par -e ./raa_tt-exp.par -p ./src/raa_tt_parser.rs -a ./src/raa_tt_grammar_trait.rs -t RaaTtGrammar -m raa_tt_grammar -g
    if let Err(err) = Builder::with_explicit_output_dir("src")
        .grammar_file("raa_tt.par")
        .expanded_grammar_output_file("../raa_tt-exp.par")
        .parser_output_file("raa_tt_parser.rs")
        .actions_output_file("raa_tt_grammar_trait.rs")
        .enable_auto_generation()
        .user_type_name("RaaTtGrammar")
        .user_trait_module_name("raa_tt_grammar")
        .trim_parse_tree()
        .generate_parser()
    {
        ParolErrorReporter::report_error(&err, "raa_tt.par").unwrap_or_default();
        process::exit(1);
    }
}
