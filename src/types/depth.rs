use super::{
    translater::{Rename, Renamer},
    Translater,
};
use bedrs::traits::IntervalBounds;

pub struct IntervalDepth<'a, I, N>
where
    I: IntervalBounds<usize, usize>,
    N: IntervalBounds<&'a str, usize>,
{
    pub iv: I,
    pub n_overlaps: usize,
    pub translater: Option<&'a Translater>,
    phantom: std::marker::PhantomData<N>,
}
impl<'a, I, N> IntervalDepth<'a, I, N>
where
    I: IntervalBounds<usize, usize>,
    N: IntervalBounds<&'a str, usize>,
    Renamer: Rename<'a, I, N>,
{
    pub fn new(iv: I, n_overlaps: usize, translater: Option<&'a Translater>) -> Self {
        Self {
            iv,
            n_overlaps,
            translater,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn get_tuple(&self) -> (&I, usize) {
        (&self.iv, self.n_overlaps)
    }
    pub fn get_named_tuple(&self) -> (N, usize) {
        if let Some(translater) = self.translater {
            let n = Renamer::rename_with(&self.iv, translater);
            (n, self.n_overlaps)
        } else {
            panic!("Translater was not provided but get_named_tuple was called - there is a bug somewhere!")
        }
    }
}
