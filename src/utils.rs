use bedrs::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    Container,
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
pub fn sort_pairs<C, T, I>(
    set_a: &mut impl Container<C, T, I>,
    set_b: &mut impl Container<C, T, I>,
    sorted: bool,
) where
    C: ChromBounds,
    T: ValueBounds,
    I: IntervalBounds<C, T>,
{
    if !sorted {
        set_a.sort();
        set_b.sort();
    } else {
        set_a.set_sorted();
        set_b.set_sorted();
    }
}
