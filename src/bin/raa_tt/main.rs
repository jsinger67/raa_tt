extern crate parol_runtime;

mod arguments;

use anyhow::{Context, Result};
use clap::Parser;
use parol_runtime::{log::debug, Report};
use raa_tt::prover::Prover;
use raa_tt::raa_tt_grammar::RaaTtGrammar;
use raa_tt::raa_tt_parser::parse;
use raa_tt::{proposition::Proposition, table_generator::TableGenerator};
use std::{fs, time::Instant};

use crate::arguments::CliArgs;

// To generate on command line:
// parol -f ./raa_tt.par -e ./raa_tt-exp.par -p ./src/raa_tt_parser.rs -a ./src/raa_tt_grammar_trait.rs -t RaaTtGrammar -m raa_tt_grammar -g

struct ErrorReporter;
impl Report for ErrorReporter {}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    env_logger::init();
    debug!("env logger started");

    let (input, file_name) = if let Some(file_name) = args.file.as_ref() {
        (
            fs::read_to_string(file_name.clone())
                .with_context(|| format!("Can't read file {}", file_name.display()))?,
            file_name.to_path_buf(),
        )
    } else {
        (args.text.unwrap().to_owned(), "direct".to_owned().into())
    };

    let quiet = args.quiet;

    let mut raa_tt_grammar = RaaTtGrammar::new();
    let now = Instant::now();
    match parse(&input, &file_name, &mut raa_tt_grammar) {
        Ok(_) => {
            let elapsed_time = now.elapsed();
            if !quiet {
                println!("{}", "-".repeat(80));
                println!("Parsing took {} milliseconds.", elapsed_time.as_millis());
                println!();
            }
            for p in &raa_tt_grammar.raa_tt.as_ref().unwrap().raa_tt_list {
                let proposition: Proposition = (&*p.biconditional).into();
                if !quiet {
                    println!("{}", "-".repeat(80));
                }

                let solver = Prover::new();
                let solve_result = solver.prove(&proposition);
                match solve_result {
                    Ok(r) => {
                        println!();
                        println!("{proposition} is {r}");
                    }
                    Err(e) => println!("Error occurred: {e}"),
                }

                if args.truth_table {
                    let table_generator = TableGenerator::new();
                    table_generator.generate_truth_table(&proposition)?;
                }
            }
            Ok(())
        }
        Err(e) => ErrorReporter::report_error(&e, file_name),
    }
}
