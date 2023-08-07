mod bed;
mod fasta;
mod general;
pub use bed::{read_genome, read_name_map, read_set, write_records, write_records_iter, write_set};
pub use fasta::{FastaIndex, IndexedFasta};
pub use general::{match_input, match_output};
