mod depth;
mod formats;
mod pairs;
mod translate;
use bedrs::{Bed12, Bed3, Bed4, Bed6, IntervalContainer, MetaInterval};
pub use depth::IntervalDepth;
pub use formats::{FieldFormat, Genome, InputFormat};
pub use pairs::IntervalPair;
pub use translate::{
    Rename, Renamer, Reorder, Retranslater, StreamTranslater, Translate, Translater,
};

pub type NumericBed3 = Bed3<usize, usize>;
pub type NamedBed3<'a> = Bed3<&'a str, usize>;
pub type Bed3Set = IntervalContainer<NumericBed3, usize, usize>;

pub type NumericBed4 = Bed4<usize, usize, usize>;
pub type NamedBed4<'a> = Bed4<&'a str, usize, &'a str>;
pub type Bed4Set = IntervalContainer<NumericBed4, usize, usize>;

pub type NumericBed6 = Bed6<usize, usize, usize, f64>;
pub type NamedBed6<'a> = Bed6<&'a str, usize, &'a str, f64>;
pub type Bed6Set = IntervalContainer<NumericBed6, usize, usize>;

pub type NumericBed12 = Bed12<usize, usize, usize, f64, usize, usize, usize, usize, usize>;
pub type NamedBed12<'a> =
    Bed12<&'a str, usize, &'a str, f64, usize, usize, &'a str, &'a str, &'a str>;
pub type Bed12Set = IntervalContainer<NumericBed12, usize, usize>;

pub type NumericMetaInterval = MetaInterval<usize, usize, usize>;
pub type NamedMetaInterval<'a> = MetaInterval<&'a str, usize, &'a str>;
pub type MetaIntervalSet = IntervalContainer<NumericMetaInterval, usize, usize>;
