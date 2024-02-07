use crate::types::{FieldFormat, InputFormat};
use clap::Subcommand;

use super::{
    ClosestArgs, ComplementArgs, CoverageArgs, ExtendArgs, FlankArgs, GetFastaArgs, IntersectArgs,
    MergeArgs, RandomArgs, SampleArgs,
};

#[derive(Subcommand)]
pub enum Command {
    /// Finds the closest interval in a secondary BED file for all intervals in a primary BED file
    Closest(ClosestArgs),

    /// Generates the complement of a BED file
    ///
    /// This reports the regions that are not covered by the input
    /// BED file but excludes regions preceding the first interval
    /// and following the last interval.
    Complement(ComplementArgs),

    /// Calculates the coverage of intervals in Set A by intervals in Set B
    Coverage(CoverageArgs),

    /// Extends the intervals of a BED file
    ///
    /// The extension is either done on both sides at once
    /// or on the left and right side separately
    Extend(ExtendArgs),

    /// Flanks the intervals of a BED file
    ///
    /// This will crefate two new flanking intervals for each interval
    /// in the input file, one on the left and one on the right side.
    Flank(FlankArgs),

    /// Extracts FASTA sequences using intervals from a BED file
    GetFasta(GetFastaArgs),

    /// Intersects two BED files
    Intersect(IntersectArgs),

    /// Merges intervals of a BED file with overlapping regions
    Merge(MergeArgs),

    /// Generates a random BED file given some parameterizations
    Random(RandomArgs),

    /// Randomly sample a BED file
    Sample(SampleArgs),

    /// Shifts the intervals of a BED file by a specified amount
    Shift {
        /// Input BED file to shift (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Path to genome file to use for bounds when shifting
        #[clap(short, long)]
        genome: Option<String>,

        /// Amount to shift intervals by (negative values shift to the left)
        #[clap(short, long, allow_hyphen_values = true)]
        amount: f64,

        /// Interprets the amount as a fraction of the interval length
        ///
        /// i.e. if the amount is 0.5, the interval will be shifted
        /// by half of its length. if the amount is 2, the interval
        /// will be shifted by twice its length.
        #[clap(short, long)]
        percent: bool,

        /// Input file format
        #[clap(short = 'T', long)]
        input_format: Option<InputFormat>,

        /// Allow for non-integer chromosome names
        #[clap(short = 'N', long)]
        field_format: Option<FieldFormat>,
    },

    /// Sorts a BED file by chromosome, start, and end
    Sort {
        /// Input GIA file to sort (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output GIA file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Input file format
        #[clap(short = 'T', long)]
        input_format: Option<InputFormat>,

        /// Allow for non-integer chromosome names
        #[clap(short = 'N', long)]
        field_format: Option<FieldFormat>,

        /// Number of threads to use for sorting (default=1)
        #[clap(short = 't', long, default_value = "1")]
        threads: usize,
    },

    /// Subtracts two BED files
    ///
    /// Will remove subtract `b` from `a`
    Subtract {
        /// Input BED file to subtract from (default=stdin)
        #[clap(short, long)]
        a: Option<String>,

        /// Secondary BED file to subtract with
        #[clap(short, long)]
        b: String,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Minimum fraction of a's interval that must be covered by b's interval
        #[clap(short = 'f', long)]
        fraction_query: Option<f64>,

        /// Minimum fraction of b's interval that must be covered by a's interval
        #[clap(short = 'F', long)]
        fraction_target: Option<f64>,

        /// Require that the fraction provided with `-f` is reciprocal to both
        /// query and target
        #[clap(
            short,
            long,
            requires = "fraction_query",
            conflicts_with = "fraction_target"
        )]
        reciprocal: bool,

        /// Requires that either fraction provided with `-f` or `-F` is met
        #[clap(short, long, requires_all=&["fraction_query", "fraction_target"], conflicts_with = "reciprocal")]
        either: bool,

        /// Keep the query records unmerged (i.e. report all subtractions)
        ///
        /// By default, the query records are merged to remove overlapping
        /// regions.
        #[clap(short, long)]
        unmerged: bool,
    },

    /// Finds all the overlapping intervals in Set B after adding a window around all
    /// intervals in Set A
    Window {
        /// Input BED file to subtract from (default=stdin)
        #[clap(short, long)]
        a: Option<String>,

        /// Secondary BED file to subtract with
        #[clap(short, long)]
        b: String,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// windows intervals on both sides by the same provided amount
        #[clap(short = 'w', long, required_unless_present_any(["left", "right"]), conflicts_with_all(&["left", "right"]))]
        both: Option<usize>,

        /// windows intervals on the left side by the provided amount
        #[clap(short, long, required_unless_present_any(["both", "right"]))]
        left: Option<usize>,

        /// windows intervals on the right side by the provided amount
        #[clap(short, long, required_unless_present_any(["both", "left"]))]
        right: Option<usize>,

        /// Only report the intervals in the query that do not overlap with the target
        /// (i.e. the inverse of the intersection)
        #[clap(short = 'v', long)]
        inverse: bool,
    },
}
