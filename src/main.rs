mod cli;
mod commands;
mod io;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use commands::sort;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        cli::Command::Sort { input, output } => sort(input, output)?,
    }
    Ok(())
}
