use dashmap::DashMap;

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
