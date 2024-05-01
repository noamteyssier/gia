pub mod bed_reader;
pub mod iter;
pub mod macros;
pub mod named;
pub mod utils;

pub use bed_reader::BedReader;
pub use iter::iter_unnamed;
pub use utils::build_reader;

pub use named::{
    read_bed12_set_named, read_bed3_set_named, read_bed4_set_named, read_bed6_set_named,
    read_bedgraph_set_named, read_gtf_set_named, read_meta_interval_set_named,
};

use crate::{
    create_set_io,
    types::{
        Bed12Set, Bed3Set, Bed4Set, Bed6Set, BedGraphSet, GtfSet, MetaIntervalSet, NumericBed12,
        NumericBed3, NumericBed4, NumericBed6, NumericBedGraph, NumericGtf, NumericMetaInterval,
        SplitTranslater,
    },
};
use anyhow::Result;
use std::io::Read;

create_set_io!(bed3, Bed3Set, NumericBed3);
create_set_io!(bed4, Bed4Set, NumericBed4);
create_set_io!(bed6, Bed6Set, NumericBed6);
create_set_io!(bed12, Bed12Set, NumericBed12);
create_set_io!(gtf, GtfSet, NumericGtf);
create_set_io!(bedgraph, BedGraphSet, NumericBedGraph);
create_set_io!(meta_interval, MetaIntervalSet, NumericMetaInterval);
