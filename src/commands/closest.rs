use crate::{
    io::{
        match_input, match_output, read_set, read_two_named_sets, write_pairs_iter_with, NameIndex,
    },
    types::IntervalPair,
};
use anyhow::Result;
use bedrs::{Closest, Container, GenomicIntervalSet};

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

#[derive(Debug, PartialEq)]
enum ClosestType {
    Upstream,
    Downstream,
    Both,
}

pub fn closest(
    a: Option<String>,
    b: String,
    output: Option<String>,
    upstream: bool,
    downstream: bool,
    named: bool,
) -> Result<()> {
    let (a_set, b_set, name_index) = load_pairs(a, Some(b), named)?;
    let closest_method = if upstream {
        ClosestType::Upstream
    } else if downstream {
        ClosestType::Downstream
    } else {
        ClosestType::Both
    };

    let pairs_iter = a_set
        .iter()
        .map(|query| {
            let target = match closest_method {
                ClosestType::Both => b_set.closest(query),
                ClosestType::Upstream => b_set.closest_upstream(query),
                ClosestType::Downstream => b_set.closest_downstream(query),
            }
            .expect("Could build closest index");
            (query, target)
        })
        .map(|(query, target)| IntervalPair::new(query.clone(), target.cloned()));

    let output_handle = match_output(output)?;
    write_pairs_iter_with(pairs_iter, output_handle, name_index.as_ref())?;
    Ok(())
}
