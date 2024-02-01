use bedrs::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    types::QueryMethod,
    IntervalContainer,
};
use rand::{thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

pub fn build_rng(seed: Option<usize>) -> Box<dyn RngCore> {
    match seed {
        Some(seed) => Box::new(ChaChaRng::seed_from_u64(seed as u64)),
        None => Box::new(thread_rng()),
    }
}

/// Sorts two sets if they are not already sorted
pub fn sort_pairs<Ia, Ib, C, T>(
    set_a: &mut IntervalContainer<Ia, C, T>,
    set_b: &mut IntervalContainer<Ib, C, T>,
    sorted: bool,
) where
    Ia: IntervalBounds<C, T>,
    Ib: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    if !sorted {
        set_a.sort();
        set_b.sort();
    } else {
        set_a.set_sorted();
        set_b.set_sorted();
    }
}

pub fn assign_query_method(
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
