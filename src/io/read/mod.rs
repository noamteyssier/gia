use crate::types::NumericBed6Set;
use bedrs::GenomicIntervalSet;

pub mod bed3;
pub mod bed6;
pub mod genome;
pub mod utils;
pub use bed3::{iter_bed3_unnamed, read_bed3_set, read_paired_bed3_sets};
pub use bed6::{read_bed6_set, read_paired_bed6_sets};
pub use genome::read_genome;
pub use utils::build_reader;

#[allow(dead_code)]
pub enum SetFormat {
    Bed3(GenomicIntervalSet<usize>),
    Bed6(NumericBed6Set),
}
