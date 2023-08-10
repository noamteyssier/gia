use anyhow::Result;
use bedrs::{Find, GenomicInterval, GenomicIntervalSet};

#[derive(Debug, Copy, Clone)]
pub enum OverlapMethod {
    Standard,
    FractionQuery(f64),
    FractionTarget(f64),
    FractionBoth(f64, f64),
    FractionEither(f64, f64),
}
impl OverlapMethod {
    pub fn from_inputs(
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

pub fn run_find<'a>(
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
