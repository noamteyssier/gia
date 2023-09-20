use anyhow::{bail, Result};
use bedrs::{traits::IntervalBounds, Container, Sample};
use serde::Serialize;

use crate::{
    io::{
        match_input, match_output, read_bed3_set, read_bed6_set, write_records_iter_with,
        WriteNamedIter, WriteNamedIterImpl,
    },
    types::{InputFormat, Translater},
    utils::build_rng,
};

fn sample_from_set<I>(
    set: &impl Container<usize, usize, I>,
    number: Option<usize>,
    fraction: Option<f64>,
    seed: Option<usize>,
    translater: Option<&Translater>,
    output: Option<String>,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Copy,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    // build rng
    let mut rng = build_rng(seed);

    // calculate number of intervals to sample
    let num = if let Some(n) = number {
        n
    } else if let Some(f) = fraction {
        if f > 1.0 {
            bail!(
                "Fraction must be less than or equal to 1.0:\n\ninput: {}",
                f
            )
        } else if f <= 0.0 {
            bail!("Fraction must be greater than 0.0:\n\ninput: {}", f)
        }
        (f * set.len() as f64) as usize
    } else {
        bail!("Must specify either number or fraction of intervals to sample")
    };

    // sample intervals as iterator
    let subset = set.sample_iter_rng(num, &mut rng)?.copied();

    // build output handle
    let output_handle = match_output(output)?;

    // write intervals to output
    write_records_iter_with(subset, output_handle, translater)?;

    Ok(())
}

pub fn sample(
    input: Option<String>,
    output: Option<String>,
    number: Option<usize>,
    fraction: Option<f64>,
    seed: Option<usize>,
    named: bool,
    format: InputFormat,
) -> Result<()> {
    // read input
    let input_handle = match_input(input)?;

    // handle input format
    match format {
        InputFormat::Bed3 => {
            let (set, translater) = read_bed3_set(input_handle, named)?;
            sample_from_set(&set, number, fraction, seed, translater.as_ref(), output)
        }
        InputFormat::Bed6 => {
            let (set, translater) = read_bed6_set(input_handle, named)?;
            sample_from_set(&set, number, fraction, seed, translater.as_ref(), output)
        }
    }
}
