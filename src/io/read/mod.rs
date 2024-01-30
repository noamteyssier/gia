pub mod bed12;
pub mod bed3;
pub mod bed6;
pub mod iter;
pub mod utils;
pub use bed12::{read_bed12_set, read_paired_bed12_sets};
pub use bed3::{read_bed3_set, read_paired_bed3_sets};
pub use bed6::{read_bed6_set, read_paired_bed6_sets};
pub use iter::iter_unnamed;
pub use utils::build_reader;

use crate::types::{Bed3Set, Bed6Set};

#[allow(dead_code)]
pub enum SetFormat {
    Bed3(Bed3Set),
    Bed6(Bed6Set),
}
