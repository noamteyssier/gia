use crate::{
    commands::{run_find, OverlapMethod},
    io::{
        match_input, match_output, read_set, read_two_named_sets, write_records_iter_with,
    }, types::Translater,
};
use anyhow::Result;
use bedrs::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    Container, GenomicIntervalSet, Merge, Subtract,
};

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

fn queued_diff<It, I, C, T>(query: &I, overlaps: It) -> Box<dyn Iterator<Item = I>>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C, T> + Copy + 'static,
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

fn iter_subtraction<'a, A, B, I, C, T>(
    aset: &'a A,
    bset: &'a B,
    method: &'a OverlapMethod,
) -> Box<dyn Iterator<Item = I> + 'a>
where
    A: Container<C, T, I> + 'a,
    B: Container<C, T, I> + 'a,
    I: IntervalBounds<C, T> + Copy + 'static,
    C: ChromBounds,
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
    let (query_set, target_set, translater) = load_pairs(query_path, Some(target_path), named)?;
    let overlap_method =
        OverlapMethod::from_inputs(fraction_query, fraction_target, reciprocal, either);

    let output_handle = match_output(output_path)?;
    let bset = target_set.merge()?;

    if unmerged {
        let sub_iter = iter_subtraction(&query_set, &bset, &overlap_method);
        write_records_iter_with(sub_iter, output_handle, translater.as_ref())?;
    } else {
        let aset = query_set.merge()?;
        let sub_iter = iter_subtraction(&aset, &bset, &overlap_method);
        write_records_iter_with(sub_iter, output_handle, translater.as_ref())?;
    }

    Ok(())
}
