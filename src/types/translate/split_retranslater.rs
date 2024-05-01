use super::{Retranslater, Translate, Translater};

pub struct SplitRetranslater {
    chr_tl: Retranslater,
    meta_tl: Translater,
}
impl SplitRetranslater {
    pub fn new(chr_tl: Retranslater, meta_tl: Translater) -> Self {
        Self { chr_tl, meta_tl }
    }
}
impl Translate for SplitRetranslater {
    fn get_chr_name(&self, idx: usize) -> Option<&str> {
        self.chr_tl.get_chr_name(idx)
    }
    fn get_meta_name(&self, idx: usize) -> Option<&str> {
        self.meta_tl.get_meta_name(idx)
    }
}
