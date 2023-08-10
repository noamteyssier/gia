mod fasta;
mod general;
mod read;
mod write;
pub use fasta::{FastaIndex, IndexedFasta};
pub use general::{match_input, match_output};
pub use read::{
    read_genome, read_name_map, read_named_set, read_set, read_set_with, read_two_named_sets,
};
pub use write::{
    write_named_records_iter, write_records, write_records_iter, write_records_iter_with,
    write_records_with, write_set, write_set_with,
};

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NamedInterval<'a> {
    pub name: &'a str,
    pub start: usize,
    pub end: usize,
}
pub type NameIndex = HashMap<usize, String>;
