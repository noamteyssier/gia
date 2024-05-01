use super::{Rename, Renamer, SplitTranslater};
use bedrs::{traits::IntervalBounds, Score};

pub struct IntervalSpacing<'a, I, N>
where
    I: IntervalBounds<usize, usize>,
    N: IntervalBounds<&'a str, usize>,
{
    pub iv: I,
    pub spacing: Score,
    pub translater: Option<&'a SplitTranslater>,
    phantom: std::marker::PhantomData<N>,
}
impl<'a, I, N> IntervalSpacing<'a, I, N>
where
    I: IntervalBounds<usize, usize>,
    N: IntervalBounds<&'a str, usize>,
    Renamer: Rename<'a, I, N>,
{
    pub fn new(iv: I, spacing: Option<isize>, translater: Option<&'a SplitTranslater>) -> Self {
        let spacing = spacing.map(|s| s as f64).into();
        Self {
            iv,
            spacing,
            translater,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn get_tuple(&self) -> (&I, Score) {
        (&self.iv, self.spacing)
    }
    pub fn get_named_tuple(&self) -> (N, Score) {
        if let Some(translater) = self.translater {
            let n = Renamer::rename_with(&self.iv, translater);
            (n, self.spacing)
        } else {
            panic!("SplitTranslater was not provided but get_named_tuple was called - there is a bug somewhere!")
        }
    }
}
