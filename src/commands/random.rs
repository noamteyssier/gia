use anyhow::Result;
use bedrs::GenomicInterval;
use rand::{thread_rng, Rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::io::{match_output, write_records_iter};

fn build_rng(seed: Option<usize>) -> Box<dyn RngCore> {
    match seed {
        Some(seed) => Box::new(ChaChaRng::seed_from_u64(seed as u64)),
        None => Box::new(thread_rng()),
    }
}

pub fn random(
    n_intervals: usize,
    l_intervals: usize,
    n_chr: usize,
    max_chr_len: usize,
    seed: Option<usize>,
    output: Option<String>,
) -> Result<()> {
    let mut rng_intervals = build_rng(seed);
    let mut rng_chr = build_rng(seed);

    let interval_gen = (0..n_intervals)
        .map(|_| rng_intervals.gen_range(l_intervals..=max_chr_len))
        .map(|x| (x - l_intervals, x))
        .map(|(x, y)| (rng_chr.gen_range(1..=n_chr), x, y))
        .map(|(c, x, y)| GenomicInterval::new(c, x, y));

    let output_handle = match_output(output)?;
    write_records_iter(interval_gen, output_handle)?;

    Ok(())
}
