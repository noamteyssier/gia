use crate::{
    io::{match_input, match_output, write_records_iter_with},
    types::{Genome, InputFormat, NumericBed6},
    utils::build_rng,
};
use anyhow::Result;
use bedrs::{GenomicInterval, Strand};
use rand::Rng;

fn build_chr_size(
    n_chr: usize,
    max_chr_len: usize,
    genome: Option<String>,
    named: bool,
) -> Result<Genome> {
    if let Some(path) = genome {
        let handle = match_input(Some(path))?;
        if named {
            Genome::from_reader_named(handle)
        } else {
            Genome::from_reader_unnamed(handle)
        }
    } else {
        Ok(Genome::from_params(n_chr, max_chr_len))
    }
}

pub fn random_bed3(
    n_intervals: usize,
    l_intervals: usize,
    n_chr: usize,
    max_chr_len: usize,
    seed: Option<usize>,
    output: Option<String>,
    genome: Option<String>,
    named: bool,
) -> Result<()> {
    let mut rng_intervals = build_rng(seed);
    let mut rng_chr = build_rng(seed);
    let genome = build_chr_size(n_chr, max_chr_len, genome, named)?;

    let interval_gen = (0..n_intervals)
        // draw a random chromosome
        .map(|_| genome.sample_chr(&mut rng_chr))
        // draw a random end position in the chromosome
        .map(|c| {
            let y = rng_intervals.gen_range(l_intervals..=genome.chr_size(c));
            (c, y)
        })
        // calculate the start position
        .map(|(c, y)| {
            let x = y - l_intervals;
            (c, x, y)
        })
        // build the interval
        .map(|(c, x, y)| GenomicInterval::new(c, x, y));

    let output_handle = match_output(output)?;
    write_records_iter_with(interval_gen, output_handle, genome.translater())?;

    Ok(())
}

pub fn random_bed6(
    n_intervals: usize,
    l_intervals: usize,
    n_chr: usize,
    max_chr_len: usize,
    seed: Option<usize>,
    output: Option<String>,
    genome: Option<String>,
    named: bool,
) -> Result<()> {
    let mut rng_intervals = build_rng(seed);
    let mut rng_chr = build_rng(seed);
    let mut rng_strand = build_rng(seed);
    let genome_sizes = build_chr_size(n_chr, max_chr_len, genome, named)?;

    let interval_gen = (0..n_intervals)
        // draw a random chromosome
        .map(|_| genome_sizes.sample_chr(&mut rng_chr))
        // draw a random end position in the chromosome
        .map(|c| {
            let y = rng_intervals.gen_range(l_intervals..=genome_sizes.chr_size(c));
            (c, y)
        })
        // draw a random strand
        .map(|(c, y)| {
            let s: Strand = rng_strand.gen();
            (c, y, s)
        })
        // calculate the start position
        .map(|(c, y, s)| {
            let x = y - l_intervals;
            (c, x, y, s)
        })
        // build the interval
        .map(|(c, x, y, s)| NumericBed6::new(c, x, y, 0, 0.0, s));

    let output_handle = match_output(output)?;
    write_records_iter_with(interval_gen, output_handle, genome_sizes.translater())?;

    Ok(())
}

pub fn random(
    n_intervals: usize,
    l_intervals: usize,
    n_chr: usize,
    max_chr_len: usize,
    seed: Option<usize>,
    output: Option<String>,
    genome: Option<String>,
    named: bool,
    format: InputFormat,
) -> Result<()> {
    match format {
        InputFormat::Bed3 => random_bed3(
            n_intervals,
            l_intervals,
            n_chr,
            max_chr_len,
            seed,
            output,
            genome,
            named,
        ),
        InputFormat::Bed6 => random_bed6(
            n_intervals,
            l_intervals,
            n_chr,
            max_chr_len,
            seed,
            output,
            genome,
            named,
        ),
    }
}
