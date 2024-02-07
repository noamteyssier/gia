use bedrs::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    IntervalContainer,
};

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
