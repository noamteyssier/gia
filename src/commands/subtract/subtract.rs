use anyhow::Result;
use bedrs::{GenomicIntervalSet, Container, Subtract, Merge, traits::{IntervalBounds, ValueBounds}};
use crate::{io::{NameIndex, match_input, read_two_named_sets, read_set, write_records_iter_with, match_output}, commands::{OverlapMethod, run_find}};

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

fn queued_diff<It, I, T>(query: &I, overlaps: It) -> Box<dyn Iterator<Item = I>>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<T> + Copy + 'static,
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

fn iter_subtraction<'a, A, B, I, T>(aset: &'a A, bset: &'a B, method: &'a OverlapMethod) -> Box<dyn Iterator<Item = I> + 'a> 
where
    A: Container<T, I> + 'a,
    B: Container<T, I> + 'a,
    I: IntervalBounds<T> + Copy + 'static,
    T: ValueBounds,
{
    let sub_iter = aset.records().iter().flat_map(|iv| {
        let overlaps = run_find(iv, bset, *method).expect("Error in finding overlaps");
        queued_diff(iv, overlaps)
    });   
    Box::new(sub_iter)
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
) -> Result<()> {
    let (query_set, target_set, name_index) = load_pairs(query_path, Some(target_path), named)?;
    let overlap_method =
        OverlapMethod::from_inputs(fraction_query, fraction_target, reciprocal, either);

    let output_handle = match_output(output_path)?;
    let bset = target_set.merge()?;

    if unmerged {
        let sub_iter = iter_subtraction(&query_set, &bset, &overlap_method);
        write_records_iter_with(sub_iter, output_handle, name_index.as_ref())?;
    } else {
        let aset = query_set.merge()?;
        let sub_iter = iter_subtraction(&aset, &bset, &overlap_method);
        write_records_iter_with(sub_iter, output_handle, name_index.as_ref())?;
    }

    Ok(())
}