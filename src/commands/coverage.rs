use std::io::Write;

use anyhow::{bail, Result};
use bedrs::{traits::IntervalBounds, types::QueryMethod, IntervalContainer};
use serde::Serialize;

use crate::{
    cli::CoverageArgs,
    io::{write_depth_iter_with, BedReader},
    types::{InputFormat, IntervalDepth, Rename, Renamer, Translater},
    utils::sort_pairs,
};

fn run_coverage<'a, Ia, Ib, Na, W>(
    set_a: &'a mut IntervalContainer<Ia, usize, usize>,
    set_b: &'a mut IntervalContainer<Ib, usize, usize>,
    translater: Option<&'a Translater>,
    query_method: QueryMethod<usize>,
    presorted: bool,
    output_handle: W,
) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Copy + Serialize,
    Ib: IntervalBounds<usize, usize> + Copy + Serialize,
    Na: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    Renamer: Rename<'a, Ia, Na>,
{
    sort_pairs(set_a, set_b, presorted);
    let depth_iter = set_a.records().iter().map(|iv| {
        let n_overlaps = set_b
            .find_iter_sorted_method_unchecked(iv, query_method)
            .expect("Error in finding overlaps")
            .count();
        IntervalDepth::new(*iv, n_overlaps, translater)
    });
    write_depth_iter_with(depth_iter, output_handle, translater)
}

fn dispatch_coverage<W: Write>(
    reader_a: BedReader,
    reader_b: BedReader,
    output: W,
    query_method: QueryMethod<usize>,
    presorted: bool,
) -> Result<()> {
    if reader_a.is_named() != reader_b.is_named() {
        bail!("Input files must both be named or both be unnamed");
    }
    let mut translater = if reader_a.is_named() {
        Some(Translater::new())
    } else {
        None
    };
    match reader_a.input_format() {
        InputFormat::Bed3 => {
            let mut set_a = reader_a.bed3_set_with(translater.as_mut())?;
            match reader_b.input_format() {
                InputFormat::Bed3 => {
                    let mut set_b = reader_b.bed3_set_with(translater.as_mut())?;
                    run_coverage(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        query_method,
                        presorted,
                        output,
                    )
                }
                InputFormat::Bed6 => {
                    let mut set_b = reader_b.bed6_set_with(translater.as_mut())?;
                    run_coverage(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        query_method,
                        presorted,
                        output,
                    )
                }
                InputFormat::Bed12 => {
                    let mut set_b = reader_b.bed12_set_with(translater.as_mut())?;
                    run_coverage(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        query_method,
                        presorted,
                        output,
                    )
                }
            }
        }
        InputFormat::Bed6 => {
            let mut set_a = reader_a.bed6_set_with(translater.as_mut())?;
            match reader_b.input_format() {
                InputFormat::Bed3 => {
                    let mut set_b = reader_b.bed3_set_with(translater.as_mut())?;
                    run_coverage(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        query_method,
                        presorted,
                        output,
                    )
                }
                InputFormat::Bed6 => {
                    let mut set_b = reader_b.bed6_set_with(translater.as_mut())?;
                    run_coverage(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        query_method,
                        presorted,
                        output,
                    )
                }
                InputFormat::Bed12 => {
                    let mut set_b = reader_b.bed12_set_with(translater.as_mut())?;
                    run_coverage(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        query_method,
                        presorted,
                        output,
                    )
                }
            }
        }
        InputFormat::Bed12 => {
            let mut set_a = reader_a.bed12_set_with(translater.as_mut())?;
            match reader_b.input_format() {
                InputFormat::Bed3 => {
                    let mut set_b = reader_b.bed3_set_with(translater.as_mut())?;
                    run_coverage(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        query_method,
                        presorted,
                        output,
                    )
                }
                InputFormat::Bed6 => {
                    let mut set_b = reader_b.bed6_set_with(translater.as_mut())?;
                    run_coverage(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        query_method,
                        presorted,
                        output,
                    )
                }
                InputFormat::Bed12 => {
                    let mut set_b = reader_b.bed12_set_with(translater.as_mut())?;
                    run_coverage(
                        &mut set_a,
                        &mut set_b,
                        translater.as_ref(),
                        query_method,
                        presorted,
                        output,
                    )
                }
            }
        }
    }
}

pub fn coverage(args: CoverageArgs) -> Result<()> {
    let (bed_a, bed_b) = args.inputs.get_readers()?;
    let query_method = args.overlap_predicates.into();
    let output = args.output.get_handle()?;
    dispatch_coverage(bed_a, bed_b, output, query_method, args.sorted)
}
