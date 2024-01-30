use bedrs::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
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
