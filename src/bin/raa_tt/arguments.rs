use std::path::PathBuf;

use clap::Parser;

// Prover for sentences of propositional calculus
#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct CliArgs {
    /// Input file
    #[arg(short = 'f', long = "file", group = "input", required = true)]
    pub file: Option<PathBuf>,

    /// Input string
    #[arg(short = 's', group = "input", required = true)]
    pub text: Option<String>,

    /// Decreased verbosity
    #[arg(short, long)]
    pub quiet: bool,
}
