use std::path::PathBuf;

use clap::Parser;

// Prover for sentences of propositional calculus
#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct CliArgs {
    /// Input file
    #[arg(short, long, group = "input", required = true)]
    pub file: Option<PathBuf>,

    /// Input string
    #[arg(short = 's', group = "input", required = true)]
    pub text: Option<String>,

    /// Generate truth table
    #[arg(short, long)]
    pub truth_table: bool,

    /// Decrease verbosity
    #[arg(short, long)]
    pub quiet: bool,
}
