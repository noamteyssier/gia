mod iter;
mod utils;
pub use iter::{
    write_demoted_records_iter_with, write_records_iter_with, WriteNamedIter, WriteNamedIterImpl,
};
pub use utils::{
    build_writer, write_depth_iter_with, write_named_records_iter_dashmap, write_pairs_iter_with,
    write_records_iter, write_segment, write_spacing_iter_with,
};
