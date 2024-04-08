pub mod bam;
mod closest;
mod cluster;
mod commands;
mod complement;
mod coverage;
mod extend;
mod flank;
mod get_fasta;
mod growth;
mod inputs;
mod intersect;
mod merge;
mod outputs;
mod overlap_predicates;
mod random;
mod sample;
mod segment;
mod shift;
mod sort;
mod spacing;
mod subtract;
mod unionbedg;
pub mod vcf;
mod window;

pub use closest::{ClosestArgs, ClosestParams};
pub use cluster::{ClusterArgs, ClusterParams};
pub use commands::Command;
pub use complement::ComplementArgs;
pub use coverage::{CoverageArgs, CoverageParams};
pub use extend::ExtendArgs;
pub use flank::FlankArgs;
pub use get_fasta::GetFastaArgs;
pub use growth::Growth;
pub use inputs::{
    DualInput, MixedInputBam, MixedInputVcf, MultiInput, SingleInput, SingleInputBam,
};
pub use intersect::{IntersectArgs, IntersectParams, OutputMethod};
pub use merge::{MergeArgs, MergeParams};
pub use outputs::{BamOutput, Output};
pub use overlap_predicates::OverlapPredicates;
pub use random::{RandomArgs, RandomParams};
pub use sample::{SampleArgs, SampleParams};
pub use segment::{SegmentArgs, SegmentParams};
pub use shift::{ShiftArgs, ShiftParams};
pub use sort::{SortArgs, SortParams};
pub use spacing::{SpacingArgs, SpacingParams};
pub use subtract::{SubtractArgs, SubtractParams};
pub use unionbedg::{UnionBedGraphArgs, UnionBedGraphParams};
pub use window::{WindowArgs, WindowParams};

use clap::Parser;

#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    /// Subcommand to run
    #[clap(subcommand)]
    pub command: Command,
}
