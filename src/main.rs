mod cli;
mod commands;
mod io;
mod types;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use commands::{
    closest, complement, extend, get_fasta, intersect, intersect_stream, merge, name_map, random,
    sample, sort, subtract,
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Complement {
            input,
            output,
            named,
            stream,
        } => complement(input, output, named, stream)?,
        Command::Closest {
            a,
            b,
            output,
            upstream,
            downstream,
            named,
            sorted,
        } => closest(a, b, output, upstream, downstream, named, sorted)?,
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
            stream,
        } => merge(input, output, sorted, named, stream)?,
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
        } => {
            if stream {
                intersect_stream(
                    a,
                    b,
                    output,
                    fraction_query,
                    fraction_target,
                    reciprocal,
                    either,
                    named,
                )?
            } else {
                intersect(
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
                )?
            }
        }
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
            )?;
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
