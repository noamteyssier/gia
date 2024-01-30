use super::{NumericBed12, NumericBed6};
use bedrs::{traits::IntervalBounds, Coordinates, GenomicInterval, IntervalContainer};
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
impl Reorder<GenomicInterval<usize>> for GenomicInterval<usize> {
    fn reorder_translater(
        // set: &mut impl Container<usize, usize, Self>,
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
            let new_name = retranslate.get_rank(iv.name()).unwrap();
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
            let new_name = retranslate.get_rank(iv.name()).unwrap();
            let new_item_rgb = retranslate.get_rank(iv.item_rgb).unwrap();
            let new_block_sizes = retranslate.get_rank(iv.block_sizes).unwrap();
            let new_block_starts = retranslate.get_rank(iv.block_starts).unwrap();
            iv.update_chr(&new_chr);
            iv.update_name(&new_name);
            iv.update_item_rgb(&new_item_rgb);
            iv.update_block_sizes(&new_block_sizes);
            iv.update_block_starts(&new_block_starts);
        });
        retranslate
    }
}
