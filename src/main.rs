mod cli;
mod commands;
mod io;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use commands::{extend, get_fasta, intersect, merge, name_map, random, sample, sort, subtract};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Sort {
            input,
            output,
            named,
        } => sort(input, output, named)?,
        Command::Merge {
            input,
            output,
            sorted,
            named,
        } => merge(input, output, sorted, named)?,
        Command::Intersect {
            a,
            b,
            output,
            fraction_query,
            fraction_target,
            reciprocal,
            either,
            with_query,
            with_target,
            unique,
            inverse,
            named,
        } => intersect(
            a,
            b,
            output,
            fraction_query,
            fraction_target,
            reciprocal,
            either,
            with_query,
            with_target,
            unique,
            inverse,
            named,
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
            named,
        } => {
            subtract(a, b, output, fraction_query, fraction_target, reciprocal, either, unmerged, named)?;
        }
        Command::GetFasta {
            bed,
            fasta,
            output,
            map,
            threads,
            named,
        } => get_fasta(bed, &fasta, output, map, threads, named)?,
        Command::NameMap { input, output, map } => name_map(input, output, map)?,
        Command::Extend {
            input,
            output,
            both,
            left,
            right,
            genome,
            named,
        } => extend(input, output, both, left, right, genome, named)?,
        Command::Random {
            n_intervals,
            l_intervals,
            n_chr,
            max_chr_len,
            seed,
            output,
            genome,
        } => random(
            n_intervals,
            l_intervals,
            n_chr,
            max_chr_len,
            seed,
            output,
            genome,
        )?,
        Command::Sample {
            input,
            output,
            number,
            fraction,
            seed,
            named,
        } => sample(input, output, number, fraction, seed, named)?,
    }
    Ok(())
}
