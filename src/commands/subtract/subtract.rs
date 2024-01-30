use crate::{
    commands::{run_find, OverlapMethod},
    io::{
        match_input, match_output, read_paired_bed12_sets, read_paired_bed3_sets,
        read_paired_bed6_sets, write_records_iter_with, WriteNamedIter, WriteNamedIterImpl,
    },
    types::{InputFormat, Translater},
    utils::sort_pairs,
};
use anyhow::Result;
use bedrs::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    Coordinates, IntervalContainer, Subtract,
};
use serde::Serialize;
use std::{fmt::Debug, io::Write};

fn queued_diff<It, I, C, T>(query: &I, overlaps: It) -> Box<dyn Iterator<Item = I>>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C, T> + Copy + 'static + Debug,
    C: ChromBounds,
    T: ValueBounds,
{
    let mut differences = Vec::new();
    let mut num_overlaps = 0;
    for ov in overlaps {
        if differences.is_empty() {
            differences.extend(query.subtract_iter(&ov));
        } else {
            let last_difference = differences.pop().unwrap();
            differences.extend(last_difference.subtract_iter(&ov));
        }
        num_overlaps += 1;
    }
    if num_overlaps == 0 {
        Box::new(std::iter::once(query.clone()))
    } else {
        Box::new(differences.into_iter())
    }
}

fn iter_subtraction<'a, I, C, T>(
    aset: &'a IntervalContainer<I, C, T>,
    bset: &'a IntervalContainer<I, C, T>,
    method: &'a OverlapMethod,
) -> Box<dyn Iterator<Item = I> + 'a>
where
    I: IntervalBounds<C, T> + Copy + 'static + Debug,
    C: ChromBounds,
    T: ValueBounds,
{
    let sub_iter = aset.records().iter().flat_map(|iv| {
        let overlaps = run_find(iv, bset, *method).expect("Error in finding overlaps");
        queued_diff(iv, overlaps)
    });
    Box::new(sub_iter)
}

fn run_subtract<'a, I, C, T, W>(
    aset: &'a IntervalContainer<I, C, T>,
    bset: &'a IntervalContainer<I, C, T>,
    method: &'a OverlapMethod,
    unmerged: bool,
    output_handle: W,
    translater: Option<&'a Translater>,
) -> Result<()>
where
    I: IntervalBounds<C, T> + Copy + 'static + Coordinates<usize, usize> + Serialize + Debug,
    C: ChromBounds,
    T: ValueBounds,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    if unmerged {
        let sub_iter = iter_subtraction(aset, bset, method);
        write_records_iter_with(sub_iter, output_handle, translater)?;
    } else {
        let aset = aset.merge()?;
        let sub_iter = iter_subtraction(&aset, bset, method);
        write_records_iter_with(sub_iter, output_handle, translater)?;
    }
    Ok(())
}

fn subtract_bed3<W: Write>(
    query_path: Option<String>,
    target_path: String,
    output_handle: W,
    overlap_method: OverlapMethod,
    unmerged: bool,
    named: bool,
) -> Result<()> {
    // load query and target sets
    let query_handle = match_input(query_path)?;
    let target_handle = match_input(Some(target_path))?;
    let (mut query_set, mut target_set, translater) =
        read_paired_bed3_sets(query_handle, target_handle, named)?;

    // sort query and target sets
    sort_pairs(&mut query_set, &mut target_set, false);

    // merge target set
    let bset = target_set.merge()?;

    // run subtraction
    run_subtract(
        &query_set,
        &bset,
        &overlap_method,
        unmerged,
        output_handle,
        translater.as_ref(),
    )
}

fn subtract_bed6<W: Write>(
    query_path: Option<String>,
    target_path: String,
    output_handle: W,
    overlap_method: OverlapMethod,
    unmerged: bool,
    named: bool,
) -> Result<()> {
    // load query and target sets
    let query_handle = match_input(query_path)?;
    let target_handle = match_input(Some(target_path))?;
    let (mut query_set, mut target_set, translater) =
        read_paired_bed6_sets(query_handle, target_handle, named)?;

    // sort query and target sets
    sort_pairs(&mut query_set, &mut target_set, false);

    // merge target set
    let bset = target_set.merge()?;

    // run subtraction
    run_subtract(
        &query_set,
        &bset,
        &overlap_method,
        unmerged,
        output_handle,
        translater.as_ref(),
    )
}

fn subtract_bed12<W: Write>(
    query_path: Option<String>,
    target_path: String,
    output_handle: W,
    overlap_method: OverlapMethod,
    unmerged: bool,
    named: bool,
) -> Result<()> {
    // load query and target sets
    let query_handle = match_input(query_path)?;
    let target_handle = match_input(Some(target_path))?;
    let (mut query_set, mut target_set, translater) =
        read_paired_bed12_sets(query_handle, target_handle, named)?;

    // sort query and target sets
    sort_pairs(&mut query_set, &mut target_set, false);

    // merge target set
    let bset = target_set.merge()?;

    // run subtraction
    run_subtract(
        &query_set,
        &bset,
        &overlap_method,
        unmerged,
        output_handle,
        translater.as_ref(),
    )
}

pub fn subtract(
    query_path: Option<String>,
    target_path: String,
    output_path: Option<String>,
    fraction_query: Option<f64>,
    fraction_target: Option<f64>,
    reciprocal: bool,
    either: bool,
    unmerged: bool,
    named: bool,
    format: InputFormat,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let overlap_method =
        OverlapMethod::from_inputs(fraction_query, fraction_target, reciprocal, either);
    let output_handle = match_output(output_path, compression_threads, compression_level)?;
    match format {
        InputFormat::Bed3 => subtract_bed3(
            query_path,
            target_path,
            output_handle,
            overlap_method,
            unmerged,
            named,
        ),
        InputFormat::Bed6 => subtract_bed6(
            query_path,
            target_path,
            output_handle,
            overlap_method,
            unmerged,
            named,
        ),
        InputFormat::Bed12 => subtract_bed12(
            query_path,
            target_path,
            output_handle,
            overlap_method,
            unmerged,
            named,
        ),
    }
}
