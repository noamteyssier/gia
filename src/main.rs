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
        Command::Sample {
            input,
            output,
            number,
            fraction,
            seed,
            input_format,
            field_format,
        } => sample(
            input,
            output,
            number,
            fraction,
            seed,
            input_format,
            field_format,
            cli.compression_threads,
            cli.compression_level,
        )?,
        Command::Shift {
            input,
            output,
            genome,
            amount,
            percent,
            input_format,
            field_format,
        } => shift(
            input,
            output,
            genome,
            amount,
            percent,
            input_format,
            field_format,
            cli.compression_threads,
            cli.compression_level,
        )?,
        Command::Sort {
            input,
            output,
            input_format,
            field_format,
            threads,
        } => sort(
            input,
            output,
            input_format,
            field_format,
            threads,
            cli.compression_threads,
            cli.compression_level,
        )?,
        Command::Subtract {
            a,
            b,
            output,
            fraction_query,
            fraction_target,
            reciprocal,
            either,
            unmerged,
        } => {
            subtract(
                a,
                b,
                output,
                fraction_query,
                fraction_target,
                reciprocal,
                either,
                unmerged,
                cli.compression_threads,
                cli.compression_level,
            )?;
        }
        Command::Window {
            a,
            b,
            output,
            both,
            left,
            right,
            inverse,
        } => window(
            a,
            b,
            output,
            both,
            left,
            right,
            inverse,
            cli.compression_threads,
            cli.compression_level,
        )?,
    }
    Ok(())
}
