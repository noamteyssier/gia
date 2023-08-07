mod cli;
mod commands;
mod io;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use commands::{intersect, merge, sort};

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
    }
    Ok(())
}
