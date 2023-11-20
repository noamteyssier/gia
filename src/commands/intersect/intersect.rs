use super::iter::{run_function, OutputMethod};
use crate::{
    commands::{run_find, OverlapMethod},
    io::{
        build_reader, match_input, match_output, read_paired_bed12_sets, read_paired_bed3_sets,
        read_paired_bed6_sets, write_named_records_iter_dashmap, write_records_iter_with,
        NamedIter, UnnamedIter, WriteNamedIter, WriteNamedIterImpl,
    },
    types::{InputFormat, StreamTranslater, Translater},
};
use anyhow::Result;
use bedrs::{
    traits::IntervalBounds, types::iterator::QueryMethod, Container, GenomicInterval,
    IntersectIter, MergeIter,
};
use serde::Serialize;
use std::io::BufRead;

fn run_intersect_set<I>(
    query_set: &impl Container<usize, usize, I>,
    target_set: &impl Container<usize, usize, I>,
    overlap_method: OverlapMethod,
    output_method: OutputMethod,
    output: Option<String>,
    translater: Option<&Translater>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    let ix_iter = query_set.records().iter().flat_map(|iv| {
        let overlaps = run_find(iv, target_set, overlap_method).expect("Error in finding overlaps");
        let intersections = run_function(iv, overlaps, output_method);
        intersections
    });
    let output_handle = match_output(output, compression_threads, compression_level)?;
    write_records_iter_with(ix_iter, output_handle, translater)?;
    Ok(())
}

fn intersect_bed3(
    a: Option<String>,
    b: String,
    output: Option<String>,
    overlap_method: OverlapMethod,
    output_method: OutputMethod,
    named: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let handle_a = match_input(a)?;
    let handle_b = match_input(Some(b))?;
    let (query_set, target_set, translater) = read_paired_bed3_sets(handle_a, handle_b, named)?;
    run_intersect_set(
        &query_set,
        &target_set,
        overlap_method,
        output_method,
        output,
        translater.as_ref(),
        compression_threads,
        compression_level,
    )
}

fn intersect_bed6(
    a: Option<String>,
    b: String,
    output: Option<String>,
    overlap_method: OverlapMethod,
    output_method: OutputMethod,
    named: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let handle_a = match_input(a)?;
    let handle_b = match_input(Some(b))?;
    let (query_set, target_set, translater) = read_paired_bed6_sets(handle_a, handle_b, named)?;
    run_intersect_set(
        &query_set,
        &target_set,
        overlap_method,
        output_method,
        output,
        translater.as_ref(),
        compression_threads,
        compression_level,
    )
}

fn intersect_bed12(
    a: Option<String>,
    b: String,
    output: Option<String>,
    overlap_method: OverlapMethod,
    output_method: OutputMethod,
    named: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let handle_a = match_input(a)?;
    let handle_b = match_input(Some(b))?;
    let (query_set, target_set, translater) = read_paired_bed12_sets(handle_a, handle_b, named)?;
    run_intersect_set(
        &query_set,
        &target_set,
        overlap_method,
        output_method,
        output,
        translater.as_ref(),
        compression_threads,
        compression_level,
    )
}

pub fn intersect_set(
    a: Option<String>,
    b: String,
    output: Option<String>,
    fraction_query: Option<f64>,
    fraction_target: Option<f64>,
    reciprocal: bool,
    either: bool,
    with_query: bool,
    with_target: bool,
    unique: bool,
    inverse: bool,
    named: bool,
    format: InputFormat,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let overlap_method =
        OverlapMethod::from_inputs(fraction_query, fraction_target, reciprocal, either);
    let output_method = OutputMethod::from_inputs(with_query, with_target, unique, inverse);
    match format {
        InputFormat::Bed3 => intersect_bed3(
            a,
            b,
            output,
            overlap_method,
            output_method,
            named,
            compression_threads,
            compression_level,
        ),
        InputFormat::Bed6 => intersect_bed6(
            a,
            b,
            output,
            overlap_method,
            output_method,
            named,
            compression_threads,
            compression_level,
        ),
        InputFormat::Bed12 => intersect_bed12(
            a,
            b,
            output,
            overlap_method,
            output_method,
            named,
            compression_threads,
            compression_level,
        ),
    }
}

pub fn intersect(
    a: Option<String>,
    b: String,
    output: Option<String>,
    fraction_query: Option<f64>,
    fraction_target: Option<f64>,
    reciprocal: bool,
    either: bool,
    with_query: bool,
    with_target: bool,
    unique: bool,
    inverse: bool,
    named: bool,
    stream: bool,
    format: InputFormat,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    if stream {
        intersect_stream(
            a,
            b,
            output,
            fraction_query,
            fraction_target,
            reciprocal,
            either,
            named,
            compression_threads,
            compression_level,
        )
    } else {
        intersect_set(
            a,
            b,
            output,
            fraction_query,
            fraction_target,
            reciprocal,
            either,
            with_query,
            with_target,
            unique,
            inverse,
            named,
            format,
            compression_threads,
            compression_level,
        )
    }
}

fn assign_method(
    fraction_query: Option<f64>,
    fraction_target: Option<f64>,
    reciprocal: bool,
    either: bool,
) -> QueryMethod<usize> {
    let fraction_target = if reciprocal {
        fraction_query
    } else {
        fraction_target
    };
    if fraction_query.is_some() && fraction_target.is_some() {
        if either {
            QueryMethod::CompareReciprocalFractionOr(
                fraction_query.unwrap(),
                fraction_target.unwrap(),
            )
        } else {
            QueryMethod::CompareReciprocalFractionAnd(
                fraction_query.unwrap(),
                fraction_target.unwrap(),
            )
        }
    } else if fraction_query.is_some() {
        QueryMethod::CompareByQueryFraction(fraction_query.unwrap())
    } else if fraction_target.is_some() {
        QueryMethod::CompareByTargetFraction(fraction_target.unwrap())
    } else {
        QueryMethod::Compare
    }
}

fn intersect_stream(
    a: Option<String>,
    b: String,
    output: Option<String>,
    fraction_query: Option<f64>,
    fraction_target: Option<f64>,
    reciprocal: bool,
    either: bool,
    named: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let query_handle = match_input(a)?;
    let target_handle = match_input(Some(b))?;
    let mut query_csv = build_reader(query_handle);
    let mut target_csv = build_reader(target_handle);
    let output_handle = match_output(output, compression_threads, compression_level)?;
    let method = assign_method(fraction_query, fraction_target, reciprocal, either);

    if named {
        let translater = StreamTranslater::new();
        let query_iter: NamedIter<'_, '_, Box<dyn BufRead>, GenomicInterval<usize>> =
            NamedIter::new(&mut query_csv, &translater);
        let target_iter = NamedIter::new(&mut target_csv, &translater);
        let merged_query_iter = MergeIter::new(query_iter);
        let merged_target_iter = MergeIter::new(target_iter);
        let intersect_iter =
            IntersectIter::new_with_method(merged_query_iter, merged_target_iter, method);
        write_named_records_iter_dashmap(intersect_iter, output_handle, &translater)?;
    } else {
        let query_iter: UnnamedIter<'_, Box<dyn BufRead>, GenomicInterval<usize>> =
            UnnamedIter::new(&mut query_csv);
        let target_iter = UnnamedIter::new(&mut target_csv);
        let merged_query_iter = MergeIter::new(query_iter);
        let merged_target_iter = MergeIter::new(target_iter);
        let intersect_iter =
            IntersectIter::new_with_method(merged_query_iter, merged_target_iter, method);
        write_records_iter_with(intersect_iter, output_handle, None::<&Translater>)?;
    }
    Ok(())
}
