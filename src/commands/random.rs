use crate::{
    io::{match_input, match_output, read_genome, write_records_iter},
    utils::build_rng,
};
use anyhow::Result;
use bedrs::GenomicInterval;
use hashbrown::HashMap;
use rand::{seq::IteratorRandom, Rng};

fn build_chr_size(
    n_chr: usize,
    max_chr_len: usize,
    genome: Option<String>,
) -> Result<HashMap<usize, usize>> {
    if let Some(path) = genome {
        let handle = match_input(Some(path))?;
        let genome_sizes = read_genome(handle)?;
        Ok(genome_sizes)
    } else {
        let map = (0..n_chr)
            .map(|x| (x + 1, max_chr_len))
            .collect::<HashMap<usize, usize>>();
        Ok(map)
    }
}

pub fn random(
    n_intervals: usize,
    l_intervals: usize,
    n_chr: usize,
    max_chr_len: usize,
    seed: Option<usize>,
    output: Option<String>,
    genome: Option<String>,
) -> Result<()> {
    let mut rng_intervals = build_rng(seed);
    let mut rng_chr = build_rng(seed);
    let genome_sizes = build_chr_size(n_chr, max_chr_len, genome)?;

    let interval_gen = (0..n_intervals)
        // draw a random chromosome
        .map(|_| genome_sizes.keys().choose(&mut rng_chr).unwrap())
        // draw a random end position in the chromosome
        .map(|c| {
            let y = rng_intervals.gen_range(l_intervals..=genome_sizes[c]);
            (c, y)
        })
        // calculate the start position
        .map(|(c, y)| {
            let x = y - l_intervals;
            (c, x, y)
        })
        // build the interval
        .map(|(c, x, y)| GenomicInterval::new(*c, x, y));

    let output_handle = match_output(output)?;
    write_records_iter(interval_gen, output_handle)?;

    Ok(())
}
