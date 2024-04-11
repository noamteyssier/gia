use super::{BamCoverageArgs, ConvertArgs, FilterArgs};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub enum BamCommand {
    /// Convert BAM to different formats
    Convert(ConvertArgs),

    /// Measure coverage of BAM records over interval regions
    Coverage(BamCoverageArgs),

    /// Filter BAM records based on overlap criteria to other regions
    Filter(FilterArgs),
}
