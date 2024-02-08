mod general;
mod iter;
pub mod read;
mod write;
pub use general::{match_input, match_output};
pub use iter::{NamedIter, UnnamedIter};
pub use read::{build_reader, iter_unnamed, read_bed3_set, BedReader};
use serde::{Deserialize, Serialize};
pub use write::{
    write_3col_iter_with, write_depth_iter_with, write_named_records_iter_dashmap,
    write_pairs_iter_with, write_records_iter, write_records_iter_with, WriteNamedIter,
    WriteNamedIterImpl,
};

#[derive(Deserialize, Serialize)]
pub struct NamedInterval<'a> {
    pub name: &'a str,
    pub start: usize,
    pub end: usize,
}

#[derive(Deserialize, Serialize)]
pub struct NamedPair<'a> {
    pub name_a: &'a str,
    pub start_a: usize,
    pub end_a: usize,
    pub name_b: Option<&'a str>,
    pub start_b: Option<usize>,
    pub end_b: Option<usize>,
}

#[derive(Deserialize, Serialize)]
pub struct UnnamedPair {
    pub name_a: usize,
    pub start_a: usize,
    pub end_a: usize,
    pub name_b: Option<usize>,
    pub start_b: Option<usize>,
    pub end_b: Option<usize>,
}
