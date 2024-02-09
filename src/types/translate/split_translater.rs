use super::{Retranslater, Translate, TranslateGroup, Translater};
use hashbrown::HashMap;

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
    pub fn has_name(&self, name: &str, group: TranslateGroup) -> bool {
        match group {
            TranslateGroup::Chr => self.chr_tl.has_name(name),
            TranslateGroup::Meta => self.meta_tl.has_name(name),
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
    pub fn get_name_to_idx(&self, group: TranslateGroup) -> &HashMap<String, usize> {
        match group {
            TranslateGroup::Chr => self.chr_tl.get_name_to_idx(),
            TranslateGroup::Meta => self.meta_tl.get_name_to_idx(),
        }
    }
    pub fn lex_sort(self, group: TranslateGroup) -> Retranslater {
        match group {
            TranslateGroup::Chr => self.chr_tl.lex_sort(),
            TranslateGroup::Meta => self.meta_tl.lex_sort(),
        }
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
