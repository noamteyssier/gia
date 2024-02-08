use crate::{
    cli::{SampleArgs, SampleParams},
    io::{write_records_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::{InputFormat, Translater},
};
use anyhow::{bail, Result};
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;
use std::io::Write;

fn sample_from_set<I, W>(
    set: &mut IntervalContainer<I, usize, usize>,
    translater: Option<&Translater>,
    params: SampleParams,
    writer: W,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Copy,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    // build rng
    let mut rng = params.build_rng();

    // calculate number of intervals to sample
    let num = if let Some(n) = params.number {
        n
    } else if let Some(f) = params.fraction {
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

    // write intervals to output
    write_records_iter_with(subset, writer, translater)
}

pub fn sample(args: SampleArgs) -> Result<()> {
    // read input
    let reader = args.input.get_reader()?;

    // open output
    let writer = args.output.get_writer()?;

    // handle input format
    match reader.input_format() {
        InputFormat::Bed3 => {
            let (mut set, translater) = reader.bed3_set()?;
            sample_from_set(&mut set, translater.as_ref(), args.params, writer)
        }
        InputFormat::Bed4 => {
            let (mut set, translater) = reader.bed4_set()?;
            sample_from_set(&mut set, translater.as_ref(), args.params, writer)
        }
        InputFormat::Bed6 => {
            let (mut set, translater) = reader.bed6_set()?;
            sample_from_set(&mut set, translater.as_ref(), args.params, writer)
        }
        InputFormat::Bed12 => {
            let (mut set, translater) = reader.bed12_set()?;
            sample_from_set(&mut set, translater.as_ref(), args.params, writer)
        }
        InputFormat::Ambiguous => {
            let (mut set, translater) = reader.meta_interval_set()?;
            sample_from_set(&mut set, translater.as_ref(), args.params, writer)
        }
    }
}
