use super::{Translate, TranslateGroup, Translater};

pub struct SplitTranslater {
    chr_tl: Translater,
    meta_tl: Translater,
}
impl SplitTranslater {
    pub fn new() -> Self {
        Self {
            chr_tl: Translater::new(),
            meta_tl: Translater::new(),
        }
    }
    pub fn add_name(&mut self, name: &str, group: TranslateGroup) {
        match group {
            TranslateGroup::Chr => self.chr_tl.add_name(name),
            TranslateGroup::Meta => self.meta_tl.add_name(name),
        }
    }
    pub fn get_idx(&self, name: &str, group: TranslateGroup) -> Option<usize> {
        match group {
            TranslateGroup::Chr => self.chr_tl.get_idx(name),
            TranslateGroup::Meta => self.meta_tl.get_idx(name),
        }
    }
    pub fn get_chr_idx(&self, name: &str) -> Option<usize> {
        self.chr_tl.get_idx(name)
    }
    pub fn get_translater(&self, group: TranslateGroup) -> &Translater {
        match group {
            TranslateGroup::Chr => &self.chr_tl,
            TranslateGroup::Meta => &self.meta_tl,
        }
    }
    pub fn disband(self) -> (Translater, Translater) {
        (self.chr_tl, self.meta_tl)
    }
}
impl Translate for SplitTranslater {
    fn get_chr_name(&self, idx: usize) -> Option<&str> {
        self.chr_tl.get_chr_name(idx)
    }
    fn get_meta_name(&self, idx: usize) -> Option<&str> {
        self.meta_tl.get_meta_name(idx)
    }
}
