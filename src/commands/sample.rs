use anyhow::{bail, Result};
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;

use crate::{
    io::{
        match_output, read_bed12_set, read_bed3_set, read_bed6_set, write_records_iter_with,
        BedReader, WriteNamedIter, WriteNamedIterImpl,
    },
    types::{FieldFormat, InputFormat, Translater},
    utils::build_rng,
};

fn sample_from_set<I>(
    set: &mut IntervalContainer<I, usize, usize>,
    number: Option<usize>,
    fraction: Option<f64>,
    seed: Option<usize>,
    translater: Option<&Translater>,
    output: Option<String>,
    compression_threads: usize,
    compression_level: u32,
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
    let output_handle = match_output(output, compression_threads, compression_level)?;

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
    input_format: Option<InputFormat>,
    field_format: Option<FieldFormat>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    // read input
    let bed_reader = BedReader::from_path(input, input_format, field_format)?;
    let named = bed_reader.is_named();

    // handle input format
    match bed_reader.input_format() {
        InputFormat::Bed3 => {
            let (mut set, translater) = read_bed3_set(bed_reader.reader(), named)?;
            sample_from_set(
                &mut set,
                number,
                fraction,
                seed,
                translater.as_ref(),
                output,
                compression_threads,
                compression_level,
            )
        }
        InputFormat::Bed6 => {
            let (mut set, translater) = read_bed6_set(bed_reader.reader(), named)?;
            sample_from_set(
                &mut set,
                number,
                fraction,
                seed,
                translater.as_ref(),
                output,
                compression_threads,
                compression_level,
            )
        }
        InputFormat::Bed12 => {
            let (mut set, translater) = read_bed12_set(bed_reader.reader(), named)?;
            sample_from_set(
                &mut set,
                number,
                fraction,
                seed,
                translater.as_ref(),
                output,
                compression_threads,
                compression_level,
            )
        }
    }
}
