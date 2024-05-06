use super::{Rename, Renamer, SplitTranslater};
use bedrs::traits::IntervalBounds;

pub struct IntervalPair<'a, Ia, Ib, Na, Nb>
where
    Ia: IntervalBounds<usize, usize> + Copy,
    Ib: IntervalBounds<usize, usize> + Copy,
    Na: IntervalBounds<&'a str, usize>,
    Nb: IntervalBounds<&'a str, usize>,
    Renamer: Rename<'a, Ia, Na> + Rename<'a, Ib, Nb>,
{
    pub iv_a: Option<Ia>,
    pub iv_b: Option<Ib>,
    pub translater: Option<&'a SplitTranslater>,
    phantom_a: std::marker::PhantomData<Na>,
    phantom_b: std::marker::PhantomData<Nb>,
}
impl<'a, Ia, Ib, Na, Nb> IntervalPair<'a, Ia, Ib, Na, Nb>
where
    Ia: IntervalBounds<usize, usize> + Copy,
    Ib: IntervalBounds<usize, usize> + Copy,
    Na: IntervalBounds<&'a str, usize>,
    Nb: IntervalBounds<&'a str, usize>,
    Renamer: Rename<'a, Ia, Na> + Rename<'a, Ib, Nb>,
{
    pub fn new(iv_a: Ia, iv_b: Ib, translater: Option<&'a SplitTranslater>) -> Self {
        Self {
            iv_a: Some(iv_a),
            iv_b: Some(iv_b),
            translater,
            phantom_a: std::marker::PhantomData,
            phantom_b: std::marker::PhantomData,
        }
    }
    pub fn from_option(
        iv_a: Option<Ia>,
        iv_b: Option<Ib>,
        translater: Option<&'a SplitTranslater>,
    ) -> Self {
        Self {
            iv_a,
            iv_b,
            translater,
            phantom_a: std::marker::PhantomData,
            phantom_b: std::marker::PhantomData,
        }
    }
    pub fn get_tuple(&self) -> (Ia, Ib) {
        match (&self.iv_a, &self.iv_b) {
            (Some(a), Some(b)) => (*a, *b),
            (None, Some(b)) => {
                let mut iv_a = Ia::empty();
                iv_a.update_all(&0, &0, &0);
                (iv_a, *b)
            }
            (Some(a), None) => {
                let mut iv_b = Ib::empty();
                iv_b.update_all(&0, &0, &0);
                (*a, iv_b)
            }
            (None, None) => {
                let mut iv_a = Ia::empty();
                let mut iv_b = Ib::empty();
                iv_a.update_all(&0, &0, &0);
                iv_b.update_all(&0, &0, &0);
                (iv_a, iv_b)
            }
        }
    }
    pub fn get_named_tuple(&self) -> (Na, Nb) {
        if let Some(translater) = self.translater {
            match (&self.iv_a, &self.iv_b) {
                (Some(a), Some(b)) => {
                    let named_a = Renamer::rename_with(a, translater);
                    let named_b = Renamer::rename_with(b, translater);
                    (named_a, named_b)
                }
                (None, Some(b)) => {
                    let mut named_a = Na::empty();
                    named_a.update_all(&".", &0, &0);
                    let named_b = Renamer::rename_with(b, translater);
                    (named_a, named_b)
                }
                (Some(a), None) => {
                    let named_a = Renamer::rename_with(a, translater);
                    let mut named_b = Nb::empty();
                    named_b.update_all(&".", &0, &0);
                    (named_a, named_b)
                }
                (None, None) => {
                    let mut named_a = Na::empty();
                    named_a.update_all(&".", &0, &0);
                    let mut named_b = Nb::empty();
                    named_b.update_all(&".", &0, &0);
                    (named_a, named_b)
                }
            }
        } else {
            panic!("SplitTranslater was not provided but get_named_tuple was called - there is a bug somewhere!")
        }
    }
}
