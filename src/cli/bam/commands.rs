use super::{ConvertArgs, FilterArgs};
use clap::Parser;

#[derive(Parser, Debug)]
pub enum BamCommand {
    /// Convert BAM to different formats
    Convert(ConvertArgs),

    /// Filter BAM records based on overlap criteria to other regions
    Filter(FilterArgs),
}
