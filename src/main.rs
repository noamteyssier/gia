mod cli;
mod commands;
mod dispatch;
mod io;
mod types;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{bam::BamCommand, bcf::BcfCommand, Cli, Command};
use commands::{
    bam, bcf, closest, cluster, complement, coverage, extend, flank, get_fasta, intersect, join,
    merge, random, sample, segment, shift, sort, spacing, subtract, unionbedgraph, window,
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Bam(command) => match command {
            BamCommand::Convert(args) => bam::convert(args)?,
            BamCommand::Filter(args) => bam::filter(args)?,
            BamCommand::Coverage(args) => bam::coverage(args)?,
        },
        Command::Bcf(command) => match command {
            BcfCommand::Filter(args) => bcf::filter(args)?,
        },
        Command::Closest(args) => closest(args)?,
        Command::Cluster(args) => cluster(args)?,
        Command::Complement(args) => complement(args)?,
        Command::Coverage(args) => coverage(args)?,
        Command::Extend(args) => extend(args)?,
        Command::Flank(args) => flank(args)?,
        Command::GetFasta(args) => get_fasta(args)?,
        Command::Intersect(args) => intersect(args)?,
        Command::Join(args) => join(args)?,
        Command::Merge(args) => merge(args)?,
        Command::Random(args) => random(args)?,
        Command::Sample(args) => sample(args)?,
        Command::Segment(args) => segment(args)?,
        Command::Shift(args) => shift(args)?,
        Command::Sort(args) => sort(args)?,
        Command::Spacing(args) => spacing(args)?,
        Command::Subtract(args) => subtract(args)?,
        Command::UnionBedGraph(args) => unionbedgraph(args)?,
        Command::Window(args) => window(args)?,
    }
    Ok(())
}
