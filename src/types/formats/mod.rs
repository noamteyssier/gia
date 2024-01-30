mod formats;
mod genome;
use bedrs::{Bed12, Bed3, Bed6, IntervalContainer};
pub use formats::InputFormat;
pub use genome::Genome;

pub type NumericBed3 = Bed3<usize, usize>;
pub type NamedBed3<'a> = Bed3<&'a str, usize>;
pub type Bed3Set = IntervalContainer<NumericBed3, usize, usize>;

pub type NumericBed6 = Bed6<usize, usize, usize, f64>;
pub type NamedBed6<'a> = Bed6<&'a str, usize, &'a str, f64>;
pub type Bed6Set = IntervalContainer<NumericBed6, usize, usize>;

pub type NumericBed12 = Bed12<usize, usize, usize, f64, usize, usize, usize, usize, usize>;
pub type NamedBed12<'a> =
    Bed12<&'a str, usize, &'a str, f64, usize, usize, &'a str, &'a str, &'a str>;
pub type Bed12Set = IntervalContainer<NumericBed12, usize, usize>;
