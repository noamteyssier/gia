mod rename;
mod reorder;
mod retranslater;
mod stream_translater;
mod translater;
pub use rename::{Rename, Renamer};
pub use reorder::Reorder;
pub use retranslater::Retranslater;
pub use stream_translater::StreamTranslater;
pub use translater::Translater;

pub trait Translate {
    fn get_name(&self, idx: usize) -> Option<&str>;
}
