use crate::{
    cli::RandomArgs,
    io::{match_input, write_records_iter_with},
    types::{Genome, InputFormat, NumericBed12, NumericBed3, NumericBed6, Translater},
};
use anyhow::Result;
use bedrs::Strand;
use rand::Rng;
use std::io::Write;

fn build_chr_size(
    n_chr: usize,
    max_chr_len: usize,
    genome: Option<String>,
    named: bool,
    translater: &mut Translater,
) -> Result<Genome<'_>> {
    if let Some(path) = genome {
        let handle = match_input(Some(path))?;
        if named {
            Genome::from_reader_named(handle, translater)
        } else {
            Genome::from_reader_unnamed(handle)
        }
    } else {
        Ok(Genome::from_params(n_chr, max_chr_len))
    }
}

pub fn random_bed3<W: Write>(args: RandomArgs, writer: W) -> Result<()> {
    let mut rng_intervals = args.build_rng();
    let mut rng_chr = args.build_rng();
    let mut translater = Translater::new();
    let genome = build_chr_size(
        args.n_chr,
        args.max_chr_len,
        args.genome,
        args.named,
        &mut translater,
    )?;

    let interval_gen = (0..args.n_intervals)
        // draw a random chromosome
        .map(|_| genome.sample_chr(&mut rng_chr))
        // draw a random end position in the chromosome
        .map(|c| {
            let y = rng_intervals.gen_range(args.l_intervals..=genome.chr_size_unchecked(c));
            (c, y)
        })
        // calculate the start position
        .map(|(c, y)| {
            let x = y - args.l_intervals;
            (c, x, y)
        })
        // build the interval
        .map(|(c, x, y)| NumericBed3::new(c, x, y));

    write_records_iter_with(interval_gen, writer, genome.translater())
}

pub fn random_bed6<W: Write>(args: RandomArgs, writer: W) -> Result<()> {
    let mut rng_intervals = args.build_rng();
    let mut rng_chr = args.build_rng();
    let mut rng_strand = args.build_rng();
    let mut translater = Translater::new();
    let genome_sizes = build_chr_size(
        args.n_chr,
        args.max_chr_len,
        args.genome,
        args.named,
        &mut translater,
    )?;

    let interval_gen = (0..args.n_intervals)
        // draw a random chromosome
        .map(|_| genome_sizes.sample_chr(&mut rng_chr))
        // draw a random end position in the chromosome
        .map(|c| {
            let y = rng_intervals.gen_range(args.l_intervals..=genome_sizes.chr_size_unchecked(c));
            (c, y)
        })
        // draw a random strand
        .map(|(c, y)| {
            let s: Strand = rng_strand.gen();
            (c, y, s)
        })
        // calculate the start position
        .map(|(c, y, s)| {
            let x = y - args.l_intervals;
            (c, x, y, s)
        })
        // build the interval
        .map(|(c, x, y, s)| NumericBed6::new(c, x, y, 0, 0.0, s));

    write_records_iter_with(interval_gen, writer, genome_sizes.translater())?;

    Ok(())
}

pub fn random_bed12<W: Write>(args: RandomArgs, writer: W) -> Result<()> {
    let mut rng_intervals = args.build_rng();
    let mut rng_chr = args.build_rng();
    let mut rng_strand = args.build_rng();
    let mut rng_thick_start = args.build_rng();
    let mut rng_thick_end = args.build_rng();

    let mut translater = Translater::new();
    let genome_sizes = build_chr_size(
        args.n_chr,
        args.max_chr_len,
        args.genome,
        args.named,
        &mut translater,
    )?;

    let interval_gen = (0..args.n_intervals)
        // draw a random chromosome
        .map(|_| genome_sizes.sample_chr(&mut rng_chr))
        // draw a random end position in the chromosome
        .map(|c| {
            let y = rng_intervals.gen_range(args.l_intervals..=genome_sizes.chr_size_unchecked(c));
            (c, y)
        })
        // draw a random strand
        .map(|(c, y)| {
            let s: Strand = rng_strand.gen();
            (c, y, s)
        })
        // calculate the start position
        .map(|(c, y, s)| {
            let x = y - args.l_intervals;
            (c, x, y, s)
        })
        // draw a random thick start
        .map(|(c, x, y, s)| {
            let t = rng_thick_start.gen_range(x..=y);
            (c, x, y, t, s)
        })
        // draw a random thick end
        .map(|(c, x, y, t, s)| {
            let u = rng_thick_end.gen_range(t..=y);
            (c, x, y, t, u, s)
        })
        // build the interval
        .map(|(c, x, y, t, u, s)| NumericBed12::new(c, x, y, 0, 0.0, s, t, u, 0, 0, 0, 0));

    write_records_iter_with(interval_gen, writer, genome_sizes.translater())?;

    Ok(())
}

pub fn random(args: RandomArgs) -> Result<()> {
    let writer = args.output.get_writer()?;
    match args.format {
        InputFormat::Bed3 => random_bed3(args, writer),
        InputFormat::Bed6 => random_bed6(args, writer),
        InputFormat::Bed12 => random_bed12(args, writer),
    }
}
