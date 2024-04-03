mod depth;
mod formats;
mod pairs;
mod spacing;
mod translate;
use bedrs::{Bed12, Bed3, Bed4, Bed6, BedGraph, Gtf, IntervalContainer, MetaInterval};
pub use depth::IntervalDepth;
pub use formats::{FieldFormat, Genome, InputFormat};
pub use pairs::IntervalPair;
pub use spacing::IntervalSpacing;
pub use translate::{
    Rename, Renamer, Reorder, SplitRetranslater, SplitTranslater, StreamTranslater, Translate,
    TranslateGroup, Translater,
};

pub type NumericBed3 = Bed3<usize, usize>;
pub type NamedBed3<'a> = Bed3<&'a str, usize>;
pub type Bed3Set = IntervalContainer<NumericBed3, usize, usize>;

pub type NumericBed4 = Bed4<usize, usize, usize>;
pub type NamedBed4<'a> = Bed4<&'a str, usize, &'a str>;
pub type Bed4Set = IntervalContainer<NumericBed4, usize, usize>;

pub type NumericBedGraph = BedGraph<usize, usize>;
pub type NamedBedGraph<'a> = BedGraph<&'a str, usize>;
pub type BedGraphSet = IntervalContainer<NumericBedGraph, usize, usize>;

pub type NumericBed6 = Bed6<usize, usize, usize>;
pub type NamedBed6<'a> = Bed6<&'a str, usize, &'a str>;
pub type Bed6Set = IntervalContainer<NumericBed6, usize, usize>;

pub type NumericBed12 = Bed12<usize, usize, usize, usize, usize, usize, usize, usize>;
pub type NamedBed12<'a> = Bed12<&'a str, usize, &'a str, usize, usize, &'a str, &'a str, &'a str>;
pub type Bed12Set = IntervalContainer<NumericBed12, usize, usize>;

pub type NumericGtf = Gtf<usize, usize, usize>;
pub type NamedGtf<'a> = Gtf<&'a str, usize, &'a str>;
pub type GtfSet = IntervalContainer<NumericGtf, usize, usize>;

pub type NumericMetaInterval = MetaInterval<usize, usize, usize>;
pub type NamedMetaInterval<'a> = MetaInterval<&'a str, usize, &'a str>;
pub type MetaIntervalSet = IntervalContainer<NumericMetaInterval, usize, usize>;
