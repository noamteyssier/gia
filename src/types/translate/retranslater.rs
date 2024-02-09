use hashbrown::HashMap;

use super::Translate;

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
