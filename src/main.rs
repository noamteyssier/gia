mod cli;
mod commands;
mod io;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use commands::{get_fasta, intersect, merge, sort};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Sort { input, output } => sort(input, output)?,
        Command::Merge {
            input,
            output,
            sorted,
        } => merge(input, output, sorted)?,
        Command::Intersect { a, b, output } => intersect(a, b, output)?,
        Command::GetFasta {
            bed,
            fasta,
            output,
            threads,
        } => get_fasta(bed, &fasta, output, threads)?,
    }
    Ok(())
}
