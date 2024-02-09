use super::{Retranslater, Translate};
use hashbrown::HashMap;
use human_sort::compare;

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
