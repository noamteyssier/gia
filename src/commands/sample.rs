use anyhow::{bail, Result};
use bedrs::{Container, Sample};

use crate::{
    io::{match_input, match_output, read_set, write_records_iter},
    utils::build_rng,
};

pub fn sample(
    input: Option<String>,
    output: Option<String>,
    number: Option<usize>,
    fraction: Option<f64>,
    seed: Option<usize>,
) -> Result<()> {
    // read input
    let input_handle = match_input(input)?;

    // load interval set
    let set = read_set(input_handle)?;

    // build rng
    let mut rng = build_rng(seed);

    // calculate number of intervals to sample
    let num = if let Some(n) = number {
        n
    } else if let Some(f) = fraction {
        (f * set.len() as f64) as usize
    } else {
        bail!("Must specify either number or fraction of intervals to sample")
    };

    // sample intervals as iterator
    let subset = set.sample_iter_rng(num, &mut rng)?.copied();

    // build output handle
    let output_handle = match_output(output)?;

    // write intervals to output
    write_records_iter(subset, output_handle)?;

    Ok(())
}
