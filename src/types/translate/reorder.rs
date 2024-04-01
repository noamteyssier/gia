use super::{Retranslater, Translater};
use bedrs::{traits::IntervalBounds, IntervalContainer};

pub trait Reorder<C>
where
    C: IntervalBounds<usize, usize>,
{
    fn reorder_translater(
        set: &mut IntervalContainer<C, usize, usize>,
        translater: Translater,
    ) -> Retranslater;
}
impl<I> Reorder<I> for I
where
    I: IntervalBounds<usize, usize>,
{
    fn reorder_translater(
        set: &mut IntervalContainer<I, usize, usize>,
        translater: Translater,
    ) -> Retranslater {
        let retranslate = translater.lex_sort();
        set.apply_mut(|iv| {
            let new_chr = retranslate.get_rank(*iv.chr()).unwrap();
            iv.update_chr(&new_chr);
        });
        retranslate
    }
}
