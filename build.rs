use std::process;

use parol::{build::Builder, parol_runtime::Report, ParolErrorReporter};

fn main() {
    // CLI equivalent is:
    // parol -f ./raa_tt.par -e ./raa_tt-exp.par -p ./src/raa_tt_parser.rs -a ./src/raa_tt_grammar_trait.rs -t RaaTtGrammar -m raa_tt_grammar -b --disable-recovery
    if let Err(err) = Builder::with_explicit_output_dir("src")
        .grammar_file("raa_tt.par")
        .expanded_grammar_output_file("../raa_tt-exp.par")
        .parser_output_file("raa_tt_parser.rs")
        .actions_output_file("raa_tt_grammar_trait.rs")
        .minimize_boxed_types()
        .user_type_name("RaaTtGrammar")
        .user_trait_module_name("raa_tt_grammar")
        .trim_parse_tree()
        .disable_recovery()
        .generate_parser()
    {
        ParolErrorReporter::report_error(&err, "raa_tt.par").unwrap_or_default();
        process::exit(1);
    }
}
