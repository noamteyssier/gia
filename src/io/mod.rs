mod fasta;
mod general;
mod read;
mod write;
pub use write::{write_named_records_iter, write_records, write_records_iter, write_records_iter_with, write_set};
pub use read::{read_set, read_named_set, read_set_with, read_name_map, read_genome, read_two_named_sets};
pub use fasta::{FastaIndex, IndexedFasta};
pub use general::{match_input, match_output};

use serde::{Deserialize, Serialize};
use hashbrown::HashMap;

#[derive(Deserialize, Serialize)]
pub struct NamedInterval<'a> {
    pub name: &'a str,
    pub start: usize,
    pub end: usize,
}
pub type NameIndex = HashMap<usize, String>;
