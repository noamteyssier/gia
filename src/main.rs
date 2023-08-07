mod cli;
mod commands;
mod io;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use commands::{merge, sort};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Sort { input, output } => sort(input, output)?,
        Command::Merge { input, output } => merge(input, output)?,
    }
    Ok(())
}
