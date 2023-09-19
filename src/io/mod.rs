mod general;
mod iter;
mod read;
mod write;
pub use general::{match_input, match_output};
pub use iter::{NamedIter, UnnamedIter};
pub use read::{
    build_reader, read_genome, read_iter, read_named_set, read_set, read_set_with,
    read_two_named_sets,
};
pub use write::{
    build_writer, write_named_pairs_iter, write_named_records_iter,
    write_named_records_iter_dashmap, write_pairs_iter, write_pairs_iter_with, write_records,
    write_records_iter, write_records_iter_with, write_records_with, write_set, write_set_with,
};
use serde::{Deserialize, Serialize};

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
