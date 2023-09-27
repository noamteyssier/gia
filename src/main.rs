mod cli;
mod commands;
mod io;
mod types;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use commands::{
    closest, complement, extend, get_fasta, intersect, merge, name_map, random, sample, sort,
    subtract,
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Closest {
            a,
            b,
            output,
            upstream,
            downstream,
            named,
            format,
            sorted,
        } => closest(a, b, output, upstream, downstream, named, format, sorted)?,
        Command::Complement {
            input,
            output,
            named,
            stream,
        } => complement(input, output, named, stream)?,
        Command::Extend {
            input,
            output,
            both,
            left,
            right,
            genome,
            named,
            format,
        } => extend(input, output, both, left, right, genome, named, format)?,
        Command::GetFasta {
            bed,
            fasta,
            output,
            format,
        } => get_fasta(bed, &fasta, output, format)?,
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
            stream,
            format,
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
            stream,
            format,
        )?,
        Command::Merge {
            input,
            output,
            sorted,
            named,
            stream,
            format,
        } => merge(input, output, sorted, named, stream, format)?,
        Command::NameMap { input, output, map } => name_map(input, output, map)?,
        Command::Random {
            n_intervals,
            l_intervals,
            n_chr,
            max_chr_len,
            seed,
            output,
            genome,
            named,
            format,
        } => random(
            n_intervals,
            l_intervals,
            n_chr,
            max_chr_len,
            seed,
            output,
            genome,
            named,
            format,
        )?,
        Command::Sample {
            input,
            output,
            number,
            fraction,
            seed,
            named,
            format,
        } => sample(input, output, number, fraction, seed, named, format)?,
        Command::Sort {
            input,
            output,
            named,
            format,
        } => sort(input, output, named, format)?,
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
            format,
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
                named,
                format,
            )?;
        }
    }
    Ok(())
}
