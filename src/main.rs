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
        } => closest(
            a,
            b,
            output,
            upstream,
            downstream,
            named,
            format,
            sorted,
            cli.compression_threads,
            cli.compression_level,
        )?,
        Command::Complement {
            input,
            output,
            named,
            stream,
        } => complement(
            input,
            output,
            named,
            stream,
            cli.compression_threads,
            cli.compression_level,
        )?,
        Command::Extend {
            input,
            output,
            both,
            left,
            right,
            genome,
            named,
            format,
        } => extend(
            input,
            output,
            both,
            left,
            right,
            genome,
            named,
            format,
            cli.compression_threads,
            cli.compression_level,
        )?,
        Command::GetFasta {
            bed,
            fasta,
            output,
            format,
        } => get_fasta(
            bed,
            &fasta,
            output,
            format,
            cli.compression_threads,
            cli.compression_level,
        )?,
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
            cli.compression_threads,
            cli.compression_level,
        )?,
        Command::Merge {
            input,
            output,
            sorted,
            named,
            stream,
            format,
        } => merge(
            input,
            output,
            sorted,
            named,
            stream,
            format,
            cli.compression_threads,
            cli.compression_level,
        )?,
        Command::NameMap { input, output, map } => name_map(
            input,
            output,
            map,
            cli.compression_threads,
            cli.compression_level,
        )?,
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
            cli.compression_threads,
            cli.compression_level,
        )?,
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
                cli.compression_threads,
                cli.compression_level,
            )?;
        }
    }
    Ok(())
}
