use dashmap::DashMap;
use hashbrown::HashMap;

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
    pub fn get_name(&self, idx: usize) -> Option<&str> {
        self.idx_to_name.get(&idx).map(|s| s.as_str())
    }
    #[allow(dead_code)]
    pub fn get_name_to_idx(&self) -> &HashMap<String, usize> {
        &self.name_to_idx
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
