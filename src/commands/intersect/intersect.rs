use anyhow::Result;
use bedrs::{Container, GenomicIntervalSet};

use crate::io::{match_input, match_output, read_set, write_records_iter};

use super::{
    find::{run_find, OverlapMethod},
    iter::{run_function, OutputMethod},
};

fn load_and_sort(input: Option<String>) -> Result<GenomicIntervalSet<usize>> {
    let handle = match_input(input)?;
    let mut set = read_set(handle)?;
    set.sort();
    Ok(set)
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
) -> Result<()> {
    let a_set = load_and_sort(a)?;
    let b_set = load_and_sort(Some(b))?;
    let overlap_method =
        OverlapMethod::from_inputs(fraction_query, fraction_target, reciprocal, either);
    let output_method = OutputMethod::from_inputs(with_query, with_target, unique, inverse);

    let ix_iter = a_set
        .records()
        .iter()
        .map(|iv| {
            let overlaps = run_find(iv, &b_set, overlap_method).expect("Error in finding overlaps");
            let intersections = run_function(iv, overlaps, output_method);
            intersections
        })
        .flatten();
    let output_handle = match_output(output)?;
    write_records_iter(ix_iter, output_handle)?;
    Ok(())
}
