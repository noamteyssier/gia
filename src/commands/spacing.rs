use crate::{
    cli::{SpacingArgs, SpacingParams},
    dispatch_single,
    io::{write_spacing_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::{IntervalSpacing, NumericBed3, Rename, Renamer, SplitTranslater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, Coordinates, IntervalContainer};
use serde::Serialize;
use std::io::Write;

fn calculate_spacing<I>(iv: I, last_iv: Option<NumericBed3>) -> Option<isize>
where
    I: IntervalBounds<usize, usize> + Copy,
{
    match last_iv {
        Some(last) => {
            if !iv.bounded_chr(&last) {
                None
            } else if iv.overlaps(&last) {
                Some(-1)
            } else if iv.borders(&last) {
                Some(0)
            } else {
                // convert to isize to allow negative values
                let diff = iv.start() as isize - last.end() as isize;
                Some(diff)
            }
        }
        None => None,
    }
}

fn spacing_set<'a, I, N, W>(
    mut set: IntervalContainer<I, usize, usize>,
    translater: Option<&'a SplitTranslater>,
    params: SpacingParams,
    output: W,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Copy,
    N: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
    NumericBed3: From<I>,
    Renamer: Rename<'a, I, N>,
{
    if params.is_sorted {
        set.set_sorted();
    } else {
        set.sort();
    }
    let mut last_iv: Option<NumericBed3> = None;
    let spacing_iter = set.into_iter().map(|iv| {
        let spacing = calculate_spacing(iv, last_iv);
        last_iv = Some(iv.into());
        IntervalSpacing::new(iv, spacing, translater)
    });
    write_spacing_iter_with(spacing_iter, output, translater)
}

pub fn spacing(args: SpacingArgs) -> Result<()> {
    let reader = args.inputs.get_reader()?;
    let writer = args.output.get_writer()?;
    dispatch_single!(reader, writer, args.params, spacing_set)
}
