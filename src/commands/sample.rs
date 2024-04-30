use crate::{
    cli::{SampleArgs, SampleParams},
    dispatch_single,
    io::{write_records_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::SplitTranslater,
};
use anyhow::{bail, Result};
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;
use std::io::Write;

fn sample_from_set<I, W>(
    set: IntervalContainer<I, usize, usize>,
    translater: Option<&SplitTranslater>,
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
    let reader = args.input.get_reader()?;
    let writer = args.output.get_writer()?;
    dispatch_single!(reader, writer, args.params, sample_from_set)
}
