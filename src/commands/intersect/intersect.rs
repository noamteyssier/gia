use super::iter::{run_function, OutputMethod};
use crate::{
    commands::{run_find, OverlapMethod},
    io::{
        build_reader, match_input, match_output, read_set, read_two_named_sets,
        write_named_records_iter_dashmap, write_records_iter_with, NamedIter, UnnamedIter,
    },
    types::Translater,
};
use anyhow::Result;
use bedrs::{
    types::iterator::QueryMethod, Container, GenomicIntervalSet, IntersectIter, MergeIter,
};
use dashmap::DashMap;

fn load_pairs(
    query_input: Option<String>,
    target_input: Option<String>,
    named: bool,
) -> Result<(
    GenomicIntervalSet<usize>,
    GenomicIntervalSet<usize>,
    Option<Translater>,
)> {
    let query_handle = match_input(query_input)?;
    let target_handle = match_input(target_input)?;
    let (mut query_set, mut target_set, translater) = if named {
        let (query_set, target_set, translater) = read_two_named_sets(query_handle, target_handle)?;
        (query_set, target_set, Some(translater))
    } else {
        let query_set = read_set(query_handle)?;
        let target_set = read_set(target_handle)?;
        (query_set, target_set, None)
    };
    query_set.sort();
    target_set.sort();
    Ok((query_set, target_set, translater))
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
) -> Result<()> {
    let (query_set, target_set, translater) = load_pairs(a, Some(b), named)?;
    let overlap_method =
        OverlapMethod::from_inputs(fraction_query, fraction_target, reciprocal, either);
    let output_method = OutputMethod::from_inputs(with_query, with_target, unique, inverse);

    let ix_iter = query_set.records().iter().flat_map(|iv| {
        let overlaps =
            run_find(iv, &target_set, overlap_method).expect("Error in finding overlaps");
        let intersections = run_function(iv, overlaps, output_method);
        intersections
    });
    let output_handle = match_output(output)?;
    write_records_iter_with(ix_iter, output_handle, translater.as_ref())?;
    Ok(())
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

pub fn intersect_stream(
    a: Option<String>,
    b: String,
    output: Option<String>,
    fraction_query: Option<f64>,
    fraction_target: Option<f64>,
    reciprocal: bool,
    either: bool,
    named: bool,
) -> Result<()> {
    let query_handle = match_input(a)?;
    let target_handle = match_input(Some(b))?;
    let mut query_csv = build_reader(query_handle);
    let mut target_csv = build_reader(target_handle);
    let output_handle = match_output(output)?;
    let method = assign_method(fraction_query, fraction_target, reciprocal, either);

    if named {
        let translater = DashMap::new();
        let idx_map = DashMap::new();
        let query_iter = NamedIter::new(&mut query_csv, &translater, &idx_map);
        let target_iter = NamedIter::new(&mut target_csv, &translater, &idx_map);
        let merged_query_iter = MergeIter::new(query_iter);
        let merged_target_iter = MergeIter::new(target_iter);
        let intersect_iter =
            IntersectIter::new_with_method(merged_query_iter, merged_target_iter, method);
        write_named_records_iter_dashmap(intersect_iter, output_handle, &idx_map)?;
    } else {
        let query_iter = UnnamedIter::new(&mut query_csv);
        let target_iter = UnnamedIter::new(&mut target_csv);
        let merged_query_iter = MergeIter::new(query_iter);
        let merged_target_iter = MergeIter::new(target_iter);
        let intersect_iter =
            IntersectIter::new_with_method(merged_query_iter, merged_target_iter, method);
        write_records_iter_with(intersect_iter, output_handle, None)?;
    }
    Ok(())
}
