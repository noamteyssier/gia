use bedrs::traits::{ChromBounds, IntervalBounds, ValueBounds};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize)]
pub struct IntervalPair<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    #[serde(flatten)]
    pub iv_a: I,
    #[serde(flatten)]
    pub iv_b: Option<I>,
    #[serde(skip)]
    phantom_c: PhantomData<C>,
    #[serde(skip)]
    phantom_t: PhantomData<T>,
}
impl<I, C, T> IntervalPair<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(iv_a: I, iv_b: Option<I>) -> Self {
        Self {
            iv_a,
            iv_b,
            phantom_c: PhantomData,
            phantom_t: PhantomData,
        }
    }
}
