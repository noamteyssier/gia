mod rename;
mod reorder;
mod retranslater;
mod split_retranslater;
mod split_translater;
mod stream_translater;
mod translater;
pub use rename::{Rename, Renamer};
pub use reorder::Reorder;
pub use retranslater::Retranslater;
pub use split_retranslater::SplitRetranslater;
pub use split_translater::SplitTranslater;
pub use stream_translater::StreamTranslater;
pub use translater::Translater;

pub trait Translate {
    fn get_chr_name(&self, idx: usize) -> Option<&str>;
    fn get_meta_name(&self, idx: usize) -> Option<&str>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TranslateGroup {
    Chr,
    Meta,
}
