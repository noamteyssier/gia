use super::{
    translater::{Rename, Renamer},
    Translater,
};
use bedrs::traits::IntervalBounds;

pub struct IntervalPair<'a, Ia, Ib, Na, Nb>
where
    Ia: IntervalBounds<usize, usize>,
    Ib: IntervalBounds<usize, usize>,
    Na: IntervalBounds<&'a str, usize>,
    Nb: IntervalBounds<&'a str, usize>,
    Renamer: Rename<'a, Ia, Na> + Rename<'a, Ib, Nb>,
{
    pub iv_a: Ia,
    pub iv_b: Ib,
    pub translater: Option<&'a Translater>,
    phantom_a: std::marker::PhantomData<Na>,
    phantom_b: std::marker::PhantomData<Nb>,
}
impl<'a, Ia, Ib, Na, Nb> IntervalPair<'a, Ia, Ib, Na, Nb>
where
    Ia: IntervalBounds<usize, usize>,
    Ib: IntervalBounds<usize, usize>,
    Na: IntervalBounds<&'a str, usize>,
    Nb: IntervalBounds<&'a str, usize>,
    Renamer: Rename<'a, Ia, Na> + Rename<'a, Ib, Nb>,
{
    pub fn new(iv_a: Ia, iv_b: Ib, translater: Option<&'a Translater>) -> Self {
        Self {
            iv_a,
            iv_b,
            translater,
            phantom_a: std::marker::PhantomData,
            phantom_b: std::marker::PhantomData,
        }
    }
    pub fn get_tuple(&self) -> (&Ia, &Ib) {
        (&self.iv_a, &self.iv_b)
    }
    pub fn get_named_tuple(&self) -> (Na, Nb) {
        if let Some(translater) = self.translater {
            let named_a = Renamer::rename_with(&self.iv_a, translater);
            let named_b = Renamer::rename_with(&self.iv_b, translater);
            (named_a, named_b)
        } else {
            panic!("Translater was not provided but get_named_tuple was called - there is a bug somewhere!")
        }
    }
}
