mod cli;
mod commands;
mod io;
mod types;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use commands::{
    closest, complement, coverage, extend, flank, get_fasta, intersect, merge, random, sample,
    shift, sort, subtract, window,
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Closest(args) => closest(args)?,
        Command::Complement(args) => complement(args)?,
        Command::Coverage(args) => coverage(args)?,
        Command::Extend(args) => extend(args)?,
        Command::Flank(args) => flank(args)?,
        Command::GetFasta(args) => get_fasta(args)?,
        Command::Intersect(args) => intersect(args)?,
        Command::Merge(args) => merge(args)?,
        Command::Random(args) => random(args)?,
        Command::Sample(args) => sample(args)?,
        Command::Shift(args) => shift(args)?,
        Command::Sort(args) => sort(args)?,
        Command::Subtract(args) => subtract(args)?,
        Command::Window(args) => window(args)?,
    }
    Ok(())
}
