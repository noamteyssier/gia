use crate::io::{match_input, match_output, read_set, write_records_iter};
use anyhow::Result;
use bedrs::{Container, Find, GenomicInterval, GenomicIntervalSet, Intersect};

#[derive(Debug, Copy, Clone)]
enum OverlapMethod {
    Standard,
    FractionQuery(f64),
    FractionTarget(f64),
    FractionBoth(f64, f64),
    FractionEither(f64, f64),
}
impl OverlapMethod {
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

#[derive(Debug, Copy, Clone)]
enum OutputMethod {
    Intersection,
    Query,
    Target,
    QueryUnique,
}
impl OutputMethod {
    fn from_inputs(with_query: bool, with_target: bool, unique: bool) -> Self {
        if with_query && with_target {
            panic!("Cannot specify both query and target output")
        } else if with_query {
            if unique {
                Self::QueryUnique
            } else {
                Self::Query
            }
        } else if with_target {
            Self::Target
        } else {
            Self::Intersection
        }
    }
}

fn run_find<'a>(
    query: &'a GenomicInterval<usize>,
    target_set: &'a GenomicIntervalSet<usize>,
    method: OverlapMethod,
) -> Result<Box<dyn Iterator<Item = GenomicInterval<usize>> + 'a>> {
    match method {
        OverlapMethod::Standard => {
            let iter = target_set.find_iter_sorted_unchecked(query);
            Ok(Box::new(iter.copied()))
        }
        OverlapMethod::FractionQuery(f) => {
            let iter = target_set.find_iter_sorted_query_frac_unchecked(query, f)?;
            Ok(Box::new(iter.copied()))
        }
        OverlapMethod::FractionTarget(f) => {
            let iter = target_set.find_iter_sorted_target_frac_unchecked(query, f)?;
            Ok(Box::new(iter.copied()))
        }
        OverlapMethod::FractionBoth(f_query, f_target) => {
            let iter =
                target_set.find_iter_sorted_reciprocal_frac_unchecked(query, f_query, f_target)?;
            Ok(Box::new(iter.copied()))
        }
        OverlapMethod::FractionEither(f_query, f_target) => {
            let iter = target_set
                .find_iter_sorted_reciprocal_frac_either_unchecked(query, f_query, f_target)?;
            Ok(Box::new(iter.copied()))
        }
    }
}

fn run_function<'a, It>(
    iv: &'a GenomicInterval<usize>,
    overlapping: It,
    method: OutputMethod,
) -> Result<Box<dyn Iterator<Item = GenomicInterval<usize>> + 'a>>
where
    It: Iterator<Item = GenomicInterval<usize>> + 'a,
{
    match method {
        OutputMethod::Intersection => Ok(Box::new(iter_intersections(iv, overlapping)?)),
        OutputMethod::Query => Ok(Box::new(iter_query(iv, overlapping, false)?)),
        OutputMethod::QueryUnique => Ok(Box::new(iter_query(iv, overlapping, true)?)),
        OutputMethod::Target => Ok(Box::new(iter_targets(overlapping)?)),
    }
}

fn iter_intersections<'a, It>(
    iv: &'a GenomicInterval<usize>,
    overlapping: It,
) -> Result<impl Iterator<Item = GenomicInterval<usize>> + 'a>
where
    It: Iterator<Item = GenomicInterval<usize>> + 'a,
{
    let iter = overlapping.map(|ov| {
        let ix = match ov.intersect(iv) {
            Some(ix) => ix,
            None => {
                panic!("Failed to intersect intervals: There may be a bug in FindIter")
            }
        };
        ix
    });
    Ok(iter)
}

fn iter_query<'a, It>(
    iv: &'a GenomicInterval<usize>,
    overlapping: It,
    unique: bool,
) -> Result<Box<dyn Iterator<Item = GenomicInterval<usize>> + 'a>>
where
    It: Iterator<Item = GenomicInterval<usize>> + 'a,
{
    let iter = overlapping.map(|_| iv.clone());
    if unique {
        let iter = iter.take(1);
        Ok(Box::new(iter.into_iter()))
    } else {
        Ok(Box::new(iter))
    }
}

fn iter_targets<It>(overlapping: It) -> Result<impl Iterator<Item = GenomicInterval<usize>>>
where
    It: Iterator<Item = GenomicInterval<usize>>,
{
    let iter = overlapping.map(|ov| ov.clone());
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
    with_query: bool,
    with_target: bool,
    unique: bool,
) -> Result<()> {
    let a_set = load_and_sort(a)?;
    let b_set = load_and_sort(Some(b))?;
    let overlap_method =
        OverlapMethod::from_inputs(fraction_query, fraction_target, reciprocal, either);
    let output_method = OutputMethod::from_inputs(with_query, with_target, unique);

    let ix_iter = a_set
        .records()
        .iter()
        .map(|iv| {
            let overlaps = run_find(iv, &b_set, overlap_method).expect("Error in finding overlaps");
            let intersections =
                run_function(iv, overlaps, output_method).expect("Error in finding intersections");
            intersections
        })
        .flatten();
    let output_handle = match_output(output)?;
    write_records_iter(ix_iter, output_handle)?;
    Ok(())
}
