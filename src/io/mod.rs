mod general;
mod iter;
mod read;
mod write;
pub use general::{match_input, match_output};
pub use iter::{NamedIter, UnnamedIter};
pub use read::{
    build_reader, iter_unnamed, read_bed12_set, read_bed3_set, read_bed6_set,
    read_paired_bed12_sets, read_paired_bed3_sets, read_paired_bed6_sets,
};
use serde::{Deserialize, Serialize};
pub use write::{
    build_writer, write_named_pairs_iter, write_named_records_iter_dashmap, write_pairs_iter,
    write_pairs_iter_with, write_records_iter, write_records_iter_with, WriteIter, WriteIterImpl,
    WriteNamedIter, WriteNamedIterImpl,
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
