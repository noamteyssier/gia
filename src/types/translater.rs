use super::{
    NamedBed12, NamedBed3, NamedBed4, NamedBed6, NamedMetaInterval, NumericBed12, NumericBed3,
    NumericBed4, NumericBed6, NumericMetaInterval,
};
use bedrs::{traits::IntervalBounds, Coordinates, IntervalContainer};
use dashmap::DashMap;
use hashbrown::HashMap;
use human_sort::compare;

pub trait Translate {
    fn get_name(&self, idx: usize) -> Option<&str>;
}

pub struct Translater {
    name_to_idx: HashMap<String, usize>,
    idx_to_name: HashMap<usize, String>,
}
impl Translater {
    pub fn new() -> Self {
        Self {
            name_to_idx: HashMap::new(),
            idx_to_name: HashMap::new(),
        }
    }
    pub fn has_name(&self, name: &str) -> bool {
        self.name_to_idx.contains_key(name)
    }
    pub fn add_name(&mut self, name: &str) {
        if !self.has_name(name) {
            let idx = self.name_to_idx.len();
            self.name_to_idx.insert(name.to_string(), idx);
            self.idx_to_name.insert(idx, name.to_string());
        }
    }
    #[allow(dead_code)]
    pub fn get_idx(&self, name: &str) -> Option<usize> {
        self.name_to_idx.get(name).copied()
    }
    #[allow(dead_code)]
    pub fn get_name_to_idx(&self) -> &HashMap<String, usize> {
        &self.name_to_idx
    }
    pub fn lex_sort(self) -> Retranslater {
        let mut idx_to_rank = HashMap::with_capacity(self.idx_to_name.len());
        let mut rank_to_name = HashMap::with_capacity(self.idx_to_name.len());
        let mut ordering = self
            .idx_to_name
            .iter()
            .map(|(idx, name)| (name, idx))
            .collect::<Vec<_>>();
        ordering.sort_by(|a, b| compare(a.0, b.0));
        for (order, (name, idx)) in ordering.into_iter().enumerate() {
            rank_to_name.insert(order, name.to_string());
            idx_to_rank.insert(*idx, order);
        }
        Retranslater::new(idx_to_rank, rank_to_name)
    }
}
impl Translate for Translater {
    fn get_name(&self, idx: usize) -> Option<&str> {
        self.idx_to_name.get(&idx).map(|s| s.as_str())
    }
}

#[derive(Debug)]
pub struct Retranslater {
    idx_to_rank: HashMap<usize, usize>,
    rank_to_name: HashMap<usize, String>,
}
impl Retranslater {
    pub fn new(idx_to_rank: HashMap<usize, usize>, rank_to_name: HashMap<usize, String>) -> Self {
        Self {
            idx_to_rank,
            rank_to_name,
        }
    }

    pub fn get_rank(&self, idx: usize) -> Option<usize> {
        self.idx_to_rank.get(&idx).copied()
    }
}
impl Translate for Retranslater {
    fn get_name(&self, rank: usize) -> Option<&str> {
        self.rank_to_name.get(&rank).map(|s| s.as_str())
    }
}

pub struct StreamTranslater {
    name_to_idx: DashMap<String, usize>,
    idx_to_name: DashMap<usize, String>,
}
impl StreamTranslater {
    pub fn new() -> Self {
        Self {
            name_to_idx: DashMap::new(),
            idx_to_name: DashMap::new(),
        }
    }
    pub fn has_name(&self, name: &str) -> bool {
        self.name_to_idx.contains_key(name)
    }
    pub fn add_name(&self, name: &str) {
        if !self.has_name(name) {
            let idx = self.name_to_idx.len();
            self.name_to_idx.insert(name.to_string(), idx);
            self.idx_to_name.insert(idx, name.to_string());
        }
    }
    pub fn get_name_to_idx(&self) -> &DashMap<String, usize> {
        &self.name_to_idx
    }
    pub fn get_idx_to_name(&self) -> &DashMap<usize, String> {
        &self.idx_to_name
    }
}

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
            let new_name = retranslate.get_rank(*iv.name()).unwrap();
            iv.update_chr(&new_chr);
            iv.update_name(&new_name);
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
            let new_name = retranslate.get_rank(*iv.name()).unwrap();
            iv.update_chr(&new_chr);
            iv.update_name(&new_name);
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
            let new_name = retranslate.get_rank(*iv.name()).unwrap();
            let new_item_rgb = retranslate.get_rank(*iv.item_rgb()).unwrap();
            let new_block_sizes = retranslate.get_rank(*iv.block_sizes()).unwrap();
            let new_block_starts = retranslate.get_rank(*iv.block_starts()).unwrap();
            iv.update_chr(&new_chr);
            iv.update_name(&new_name);
            iv.update_item_rgb(&new_item_rgb);
            iv.update_block_sizes(&new_block_sizes);
            iv.update_block_starts(&new_block_starts);
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
            let new_name = retranslate.get_rank(*iv.meta()).unwrap();
            iv.update_chr(&new_chr);
            iv.update_meta(&new_name);
        });
        retranslate
    }
}

pub struct Renamer;
pub trait Rename<'a, Ia, Ib>
where
    Ia: IntervalBounds<usize, usize>,
    Ib: IntervalBounds<&'a str, usize>,
{
    fn rename_with(iv: &Ia, translater: &'a Translater) -> Ib;
}
impl<'a> Rename<'a, NumericBed3, NamedBed3<'a>> for Renamer {
    fn rename_with(iv: &NumericBed3, translater: &'a Translater) -> NamedBed3<'a> {
        let chr = translater.get_name(*iv.chr()).unwrap();
        NamedBed3::new(chr, iv.start(), iv.end())
    }
}
impl<'a> Rename<'a, NumericBed4, NamedBed4<'a>> for Renamer {
    fn rename_with(iv: &NumericBed4, translater: &'a Translater) -> NamedBed4<'a> {
        let chr = translater.get_name(*iv.chr()).unwrap();
        let name = translater.get_name(*iv.name()).unwrap();
        NamedBed4::new(chr, iv.start(), iv.end(), name)
    }
}
impl<'a> Rename<'a, NumericBed6, NamedBed6<'a>> for Renamer {
    fn rename_with(iv: &NumericBed6, translater: &'a Translater) -> NamedBed6<'a> {
        let chr = translater.get_name(*iv.chr()).unwrap();
        let name = translater.get_name(*iv.name()).unwrap();
        NamedBed6::new(
            chr,
            iv.start(),
            iv.end(),
            name,
            *iv.score(),
            iv.strand().unwrap_or_default(),
        )
    }
}
impl<'a> Rename<'a, NumericBed12, NamedBed12<'a>> for Renamer {
    fn rename_with(iv: &NumericBed12, translater: &'a Translater) -> NamedBed12<'a> {
        let chr = translater.get_name(*iv.chr()).unwrap();
        let name = translater.get_name(*iv.name()).unwrap();
        let item_rgb = translater.get_name(*iv.item_rgb()).unwrap();
        let block_sizes = translater.get_name(*iv.block_sizes()).unwrap();
        let block_starts = translater.get_name(*iv.block_starts()).unwrap();
        NamedBed12::new(
            chr,
            iv.start(),
            iv.end(),
            name,
            *iv.score(),
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
    fn rename_with(iv: &NumericMetaInterval, translater: &'a Translater) -> NamedMetaInterval<'a> {
        let chr = translater.get_name(*iv.chr()).unwrap();
        let meta = translater.get_name(*iv.meta()).unwrap();
        NamedMetaInterval::new(chr, iv.start(), iv.end(), meta)
    }
}
