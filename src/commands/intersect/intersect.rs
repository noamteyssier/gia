use super::iter::{run_function_query, run_function_target};
use crate::{
    cli::{IntersectArgs, OutputMethod},
    io::{
        build_reader, write_named_records_iter_dashmap, write_records_iter_with, BedReader,
        NamedIter, UnnamedIter, WriteNamedIter, WriteNamedIterImpl,
    },
    types::{InputFormat, NumericBed3, StreamTranslater, Translater},
};
use anyhow::{bail, Result};
use bedrs::{
    traits::IntervalBounds, types::QueryMethod, IntersectIter, IntervalContainer, MergeIter,
};
use serde::Serialize;
use std::io::Write;

pub fn intersect_sets<'a, Ia, Ib, W>(
    set_a: &'a IntervalContainer<Ia, usize, usize>,
    set_b: &'a IntervalContainer<Ib, usize, usize>,
    translater: Option<&Translater>,
    query_method: QueryMethod<usize>,
    output_method: OutputMethod,
    output_handle: W,
) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Copy + Serialize,
    Ib: IntervalBounds<usize, usize> + Copy + Serialize,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<Ia> + WriteNamedIter<Ib>,
{
    match output_method {
        // Output the target intervals
        OutputMethod::Target => {
            let ix_iter = set_a.records().iter().flat_map(|iv| {
                let overlaps = set_b
                    .find_iter_sorted_method_unchecked(iv, query_method)
                    .expect("Error in finding overlaps")
                    .cloned();
                let intersections = run_function_target(overlaps, output_method);
                intersections
            });
            write_records_iter_with(ix_iter, output_handle, translater)?;
        }
        // Output the query intervals with various formats
        _ => {
            let ix_iter = set_a.records().iter().flat_map(|iv| {
                let overlaps = set_b
                    .find_iter_sorted_method_unchecked(iv, query_method)
                    .expect("Error in finding overlaps")
                    .cloned();
                let intersections = run_function_query(iv, overlaps, output_method);
                intersections
            });
            write_records_iter_with(ix_iter, output_handle, translater)?;
        }
    }
    Ok(())
}

fn match_and_intersect_sets<W: Write>(
    reader_a: BedReader,
    reader_b: BedReader,
    output_handle: W,
    query_method: QueryMethod<usize>,
    output_method: OutputMethod,
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
            let set_a = reader_a.bed3_set_with(translater.as_mut())?;
            match reader_b.input_format() {
                InputFormat::Bed3 => {
                    let set_b = reader_b.bed3_set_with(translater.as_mut())?;
                    intersect_sets(
                        &set_a,
                        &set_b,
                        translater.as_ref(),
                        query_method,
                        output_method,
                        output_handle,
                    )?;
                }
                InputFormat::Bed6 => {
                    let set_b = reader_b.bed6_set_with(translater.as_mut())?;
                    intersect_sets(
                        &set_a,
                        &set_b,
                        translater.as_ref(),
                        query_method,
                        output_method,
                        output_handle,
                    )?;
                }
                InputFormat::Bed12 => {
                    let set_b = reader_b.bed12_set_with(translater.as_mut())?;
                    intersect_sets(
                        &set_a,
                        &set_b,
                        translater.as_ref(),
                        query_method,
                        output_method,
                        output_handle,
                    )?;
                }
            }
        }
        InputFormat::Bed6 => {
            let set_a = reader_a.bed6_set_with(translater.as_mut())?;
            match reader_b.input_format() {
                InputFormat::Bed3 => {
                    let set_b = reader_b.bed3_set_with(translater.as_mut())?;
                    intersect_sets(
                        &set_a,
                        &set_b,
                        translater.as_ref(),
                        query_method,
                        output_method,
                        output_handle,
                    )?;
                }
                InputFormat::Bed6 => {
                    let set_b = reader_b.bed6_set_with(translater.as_mut())?;
                    intersect_sets(
                        &set_a,
                        &set_b,
                        translater.as_ref(),
                        query_method,
                        output_method,
                        output_handle,
                    )?;
                }
                InputFormat::Bed12 => {
                    let set_b = reader_b.bed12_set_with(translater.as_mut())?;
                    intersect_sets(
                        &set_a,
                        &set_b,
                        translater.as_ref(),
                        query_method,
                        output_method,
                        output_handle,
                    )?;
                }
            }
        }
        InputFormat::Bed12 => {
            let set_a = reader_a.bed12_set_with(translater.as_mut())?;
            match reader_b.input_format() {
                InputFormat::Bed3 => {
                    let set_b = reader_b.bed3_set_with(translater.as_mut())?;
                    intersect_sets(
                        &set_a,
                        &set_b,
                        translater.as_ref(),
                        query_method,
                        output_method,
                        output_handle,
                    )?;
                }
                InputFormat::Bed6 => {
                    let set_b = reader_b.bed6_set_with(translater.as_mut())?;
                    intersect_sets(
                        &set_a,
                        &set_b,
                        translater.as_ref(),
                        query_method,
                        output_method,
                        output_handle,
                    )?;
                }
                InputFormat::Bed12 => {
                    let set_b = reader_b.bed12_set_with(translater.as_mut())?;
                    intersect_sets(
                        &set_a,
                        &set_b,
                        translater.as_ref(),
                        query_method,
                        output_method,
                        output_handle,
                    )?;
                }
            }
        }
    }

    Ok(())
}

pub fn intersect(args: IntersectArgs) -> Result<()> {
    if args.stream {
        intersect_stream(args)
    } else {
        let (bed_a, bed_b) = args.inputs.get_readers()?;
        let query_method = args.overlap_predicates.into();
        let output_method = args.output_predicates.try_into()?;
        let writer = args.output.get_handle()?;
        match_and_intersect_sets(bed_a, bed_b, writer, query_method, output_method)
    }
}

fn intersect_stream(args: IntersectArgs) -> Result<()> {
    let (bed_a, bed_b) = args.inputs.get_readers()?;
    let named = bed_a.is_named();
    let query_handle = bed_a.reader();
    let target_handle = bed_b.reader();
    let mut query_csv = build_reader(query_handle);
    let mut target_csv = build_reader(target_handle);
    let output_handle = args.output.get_handle()?;
    let method = args.overlap_predicates.into();

    if named {
        let translater = StreamTranslater::new();
        let query_iter: NamedIter<'_, '_, _, NumericBed3> =
            NamedIter::new(&mut query_csv, &translater);
        let target_iter = NamedIter::new(&mut target_csv, &translater);
        let merged_query_iter = MergeIter::new(query_iter);
        let merged_target_iter = MergeIter::new(target_iter);
        let intersect_iter =
            IntersectIter::new_with_method(merged_query_iter, merged_target_iter, method);
        write_named_records_iter_dashmap(intersect_iter, output_handle, &translater)?;
    } else {
        let query_iter: UnnamedIter<'_, _, NumericBed3> = UnnamedIter::new(&mut query_csv);
        let target_iter = UnnamedIter::new(&mut target_csv);
        let merged_query_iter = MergeIter::new(query_iter);
        let merged_target_iter = MergeIter::new(target_iter);
        let intersect_iter =
            IntersectIter::new_with_method(merged_query_iter, merged_target_iter, method);
        write_records_iter_with(intersect_iter, output_handle, None::<&Translater>)?;
    }
    Ok(())
}
