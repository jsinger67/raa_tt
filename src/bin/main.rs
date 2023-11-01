extern crate parol_runtime;

use anyhow::{anyhow, Context, Result};
use parol_runtime::{log::debug, Report};
use raa_tt::proposition::Proposition;
use raa_tt::raa_tt_grammar::RaaTtGrammar;
use raa_tt::raa_tt_parser::parse;
use raa_tt::solver::Solver;
use std::{env, fs, time::Instant};

// To generate on command line:
// parol -f ./raa_tt.par -e ./raa_tt-exp.par -p ./src/raa_tt_parser.rs -a ./src/raa_tt_grammar_trait.rs -t RaaTtGrammar -m raa_tt_grammar -g

struct ErrorReporter;
impl Report for ErrorReporter {}

fn main() -> Result<()> {
    env_logger::init();
    debug!("env logger started");

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file_name = args[1].clone();
        let input = fs::read_to_string(file_name.clone())
            .with_context(|| format!("Can't read file {}", file_name))?;
        let mut raa_tt_grammar = RaaTtGrammar::new();
        let now = Instant::now();
        match parse(&input, &file_name, &mut raa_tt_grammar) {
            Ok(_) => {
                let elapsed_time = now.elapsed();
                println!("Parsing took {} milliseconds.", elapsed_time.as_millis());
                let proposition: Proposition = raa_tt_grammar.raa_tt.as_ref().unwrap().into();
                println!("Parsed expression: {proposition}");

                let solver = Solver::new();
                let solve_result = solver.solve(&proposition);
                match solve_result {
                    Ok(r) => println!("{proposition} is {r}"),
                    Err(e) => println!("Error occurred: {e}"),
                }
                Ok(())
            }
            Err(e) => ErrorReporter::report_error(&e, file_name),
        }
    } else {
        Err(anyhow!("Please provide a file name as first parameter!"))
    }
}
