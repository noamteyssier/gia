use super::{SplitTranslater, Translate};
use crate::types::{
    NamedBed12, NamedBed3, NamedBed4, NamedBed6, NamedMetaInterval, NumericBed12, NumericBed3,
    NumericBed4, NumericBed6, NumericMetaInterval,
};
use bedrs::{traits::IntervalBounds, Coordinates};

pub struct Renamer;
pub trait Rename<'a, Ia, Ib>
where
    Ia: IntervalBounds<usize, usize>,
    Ib: IntervalBounds<&'a str, usize>,
{
    fn rename_with(iv: &Ia, translater: &'a SplitTranslater) -> Ib;
}
impl<'a> Rename<'a, NumericBed3, NamedBed3<'a>> for Renamer {
    fn rename_with(iv: &NumericBed3, translater: &'a SplitTranslater) -> NamedBed3<'a> {
        let chr = translater.get_chr_name(*iv.chr()).unwrap();
        NamedBed3::new(chr, iv.start(), iv.end())
    }
}
impl<'a> Rename<'a, NumericBed4, NamedBed4<'a>> for Renamer {
    fn rename_with(iv: &NumericBed4, translater: &'a SplitTranslater) -> NamedBed4<'a> {
        let chr = translater.get_chr_name(*iv.chr()).unwrap();
        let name = translater.get_meta_name(*iv.name()).unwrap();
        NamedBed4::new(chr, iv.start(), iv.end(), name)
    }
}
impl<'a> Rename<'a, NumericBed6, NamedBed6<'a>> for Renamer {
    fn rename_with(iv: &NumericBed6, translater: &'a SplitTranslater) -> NamedBed6<'a> {
        let chr = translater.get_chr_name(*iv.chr()).unwrap();
        let name = translater.get_meta_name(*iv.name()).unwrap();
        NamedBed6::new(
            chr,
            iv.start(),
            iv.end(),
            name,
            iv.score(),
            iv.strand().unwrap_or_default(),
        )
    }
}
impl<'a> Rename<'a, NumericBed12, NamedBed12<'a>> for Renamer {
    fn rename_with(iv: &NumericBed12, translater: &'a SplitTranslater) -> NamedBed12<'a> {
        let chr = translater.get_chr_name(*iv.chr()).unwrap();
        let name = translater.get_meta_name(*iv.name()).unwrap();
        let item_rgb = translater.get_meta_name(*iv.item_rgb()).unwrap();
        let block_sizes = translater.get_meta_name(*iv.block_sizes()).unwrap();
        let block_starts = translater.get_meta_name(*iv.block_starts()).unwrap();
        NamedBed12::new(
            chr,
            iv.start(),
            iv.end(),
            name,
            iv.score(),
            iv.strand().unwrap_or_default(),
            iv.thick_start(),
            iv.thick_end(),
            item_rgb,
            iv.block_count(),
            block_sizes,
            block_starts,
        )
    }
}
impl<'a> Rename<'a, NumericMetaInterval, NamedMetaInterval<'a>> for Renamer {
    fn rename_with(
        iv: &NumericMetaInterval,
        translater: &'a SplitTranslater,
    ) -> NamedMetaInterval<'a> {
        let chr = translater.get_chr_name(*iv.chr()).unwrap();
        let meta = translater.get_chr_name(*iv.meta()).unwrap();
        NamedMetaInterval::new(chr, iv.start(), iv.end(), meta)
    }
}
