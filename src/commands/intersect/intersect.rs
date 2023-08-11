use super::iter::{run_function, OutputMethod};
use crate::{io::{
    match_input, match_output, read_set, read_two_named_sets, write_records_iter_with, NameIndex,
}, commands::{OverlapMethod, run_find}};
use anyhow::Result;
use bedrs::{Container, GenomicIntervalSet};

fn load_pairs(
    query_input: Option<String>,
    target_input: Option<String>,
    named: bool,
) -> Result<(
    GenomicIntervalSet<usize>,
    GenomicIntervalSet<usize>,
    Option<NameIndex>,
)> {
    let query_handle = match_input(query_input)?;
    let target_handle = match_input(target_input)?;
    let (mut query_set, mut target_set, name_index) = if named {
        let (query_set, target_set, name_index) = read_two_named_sets(query_handle, target_handle)?;
        (query_set, target_set, Some(name_index))
    } else {
        let query_set = read_set(query_handle)?;
        let target_set = read_set(target_handle)?;
        (query_set, target_set, None)
    };
    query_set.sort();
    target_set.sort();
    Ok((query_set, target_set, name_index))
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
    let (query_set, target_set, name_index) = load_pairs(a, Some(b), named)?;
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
    write_records_iter_with(ix_iter, output_handle, name_index.as_ref())?;
    Ok(())
}
