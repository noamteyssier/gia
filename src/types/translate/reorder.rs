use super::{Retranslater, Translater};
use crate::types::{NumericBed12, NumericBed3, NumericBed4, NumericBed6, NumericMetaInterval};
use bedrs::{traits::IntervalBounds, Coordinates, IntervalContainer};

pub trait Reorder<C>
where
    C: IntervalBounds<usize, usize>,
{
    fn reorder_translater(
        set: &mut IntervalContainer<C, usize, usize>,
        translater: Translater,
    ) -> Retranslater;
}
impl Reorder<NumericBed3> for NumericBed3 {
    fn reorder_translater(
        set: &mut IntervalContainer<Self, usize, usize>,
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
impl Reorder<NumericBed4> for NumericBed4 {
    fn reorder_translater(
        set: &mut IntervalContainer<Self, usize, usize>,
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
impl Reorder<NumericBed6> for NumericBed6 {
    fn reorder_translater(
        set: &mut IntervalContainer<Self, usize, usize>,
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
impl Reorder<NumericBed12> for NumericBed12 {
    fn reorder_translater(
        set: &mut IntervalContainer<Self, usize, usize>,
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
impl Reorder<NumericMetaInterval> for NumericMetaInterval {
    fn reorder_translater(
        set: &mut IntervalContainer<Self, usize, usize>,
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
