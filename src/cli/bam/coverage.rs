use crate::cli::{MixedInputBam, Output, OverlapPredicates};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
/// Calculate coverage of BAM file over BED file (i.e. for each interval in the BED file reports
/// how many intervals in the BAM file overlap)
pub struct BamCoverageArgs {
    #[clap(flatten)]
    pub inputs: MixedInputBam,

    #[clap(flatten)]
    pub params: BamCoverageParams,

    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Parameters")]
pub struct BamCoverageParams {
    /// Assert that the intervals are presorted in BOTH files (unexpected behavior if they are
    /// not)
    #[clap(short = 'S', long)]
    pub sorted: bool,

    #[clap(flatten)]
    pub overlap_predicates: OverlapPredicates,
}
