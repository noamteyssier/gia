use crate::io::{match_input, match_output, read_set, write_records_iter};
use anyhow::Result;
use bedrs::{Container, Find, GenomicInterval, GenomicIntervalSet, Intersect};

#[derive(Debug, Copy, Clone)]
enum Method {
    Standard,
    FractionQuery(f64),
    FractionTarget(f64),
    FractionBoth(f64, f64),
    FractionEither(f64, f64),
}
impl Method {
    fn from_inputs(
        f_query: Option<f64>,
        f_target: Option<f64>,
        reciprocal: bool,
        either: bool,
    ) -> Self {
        let f_target = if reciprocal { f_query } else { f_target };
        if f_query.is_none() && f_target.is_none() {
            Self::Standard
        } else if f_query.is_some() && f_target.is_none() {
            Self::FractionQuery(f_query.unwrap())
        } else if f_query.is_none() && f_target.is_some() {
            Self::FractionTarget(f_target.unwrap())
        } else {
            if either {
                Self::FractionEither(f_query.unwrap(), f_target.unwrap())
            } else {
                Self::FractionBoth(f_query.unwrap(), f_target.unwrap())
            }
        }
    }
}

fn run_find<'a>(
    query: &'a GenomicInterval<usize>,
    target_set: &'a GenomicIntervalSet<usize>,
    method: Method,
) -> Result<Box<dyn Iterator<Item = GenomicInterval<usize>> + 'a>> {
    match method {
        Method::Standard => {
            let iter = target_set.find_iter_sorted_unchecked(query);
            Ok(Box::new(iter.copied()))
        }
        Method::FractionQuery(f) => {
            let iter = target_set.find_iter_sorted_query_frac_unchecked(query, f)?;
            Ok(Box::new(iter.copied()))
        }
        Method::FractionTarget(f) => {
            let iter = target_set.find_iter_sorted_target_frac_unchecked(query, f)?;
            Ok(Box::new(iter.copied()))
        }
        Method::FractionBoth(f_query, f_target) => {
            let iter =
                target_set.find_iter_sorted_reciprocal_frac_unchecked(query, f_query, f_target)?;
            Ok(Box::new(iter.copied()))
        }
        Method::FractionEither(f_query, f_target) => {
            let iter = target_set
                .find_iter_sorted_reciprocal_frac_either_unchecked(query, f_query, f_target)?;
            Ok(Box::new(iter.copied()))
        }
    }
}

fn run_intersections<'a, It>(iv: &'a GenomicInterval<usize>, overlapping: It) -> Result<impl Iterator<Item = GenomicInterval<usize>> + 'a>
where
    It: Iterator<Item = GenomicInterval<usize>> + 'a,
{
    let iter = overlapping.map(|ov| {
        let ix = match ov.intersect(iv) {
            Some(ix) => ix,
            None => {
                panic!("Failed to intersect intervals: There may be a bug in FindIter")
            },
        };
        ix
    });
    Ok(iter)
}

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
) -> Result<()> {
    let a_set = load_and_sort(a)?;
    let b_set = load_and_sort(Some(b))?;
    let method = Method::from_inputs(fraction_query, fraction_target, reciprocal, either);
    let ix_iter = a_set
        .records()
        .iter()
        .map(|iv| {
            let overlaps = run_find(iv, &b_set, method).expect("Error in finding overlaps");
            let intersections =
                run_intersections(iv, overlaps).expect("Error in finding intersections");
            intersections
        })
        .flatten();
    let output_handle = match_output(output)?;
    write_records_iter(ix_iter, output_handle)?;
    Ok(())
}
