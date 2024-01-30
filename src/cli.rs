use crate::types::{FieldFormat, InputFormat};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    /// Subcommand to run
    #[clap(subcommand)]
    pub command: Command,

    /// Compression threads to use for output files if applicable
    #[clap(global = true, short = 'j', long, default_value = "1")]
    pub compression_threads: usize,

    /// Compression level to use for output files if applicable
    #[clap(global = true, long, default_value = "6")]
    pub compression_level: u32,
}

#[derive(Subcommand)]
pub enum Command {
    /// Finds the closest interval in a secondary BED file for all intervals in a primary BED file
    Closest {
        /// Input BED file to find closest intervals for (default=stdin)
        #[clap(short, long)]
        a: Option<String>,

        /// Secondary BED file to find closest intervals in
        #[clap(short, long)]
        b: String,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Report only the closest upstream interval
        #[clap(short = 'u', long, conflicts_with = "downstream")]
        upstream: bool,

        /// Report only the closest downstream interval
        #[clap(short = 'd', long, conflicts_with = "upstream")]
        downstream: bool,

        /// Allow for non-integer chromosome names
        #[clap(short = 'N', long)]
        named: bool,

        /// Format of the input files
        #[clap(short = 'T', long, default_value = "bed3")]
        format: InputFormat,

        /// Specify that the input files are already sorted
        #[clap(short, long)]
        sorted: bool,
    },

    /// Generates the complement of a BED file
    ///
    /// This reports the regions that are not covered by the input
    /// BED file but excludes regions preceding the first interval
    /// and following the last interval.
    Complement {
        /// Input BED file to complement (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Allow for non-integer chromosome names
        #[clap(short = 'N', long)]
        named: bool,

        /// Stream the input file instead of loading it into memory
        ///
        /// Note that this requires the input file to be sorted
        /// and will result in undefined behavior if it is not.
        #[clap(short = 'S', long, conflicts_with = "named")]
        stream: bool,
    },

    /// Extends the intervals of a BED file
    ///
    /// The extension is either done on both sides at once
    /// or on the left and right side separately
    Extend {
        /// Input BED file to extend (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Extend intervals on both sides by the same provided amount
        #[clap(short, long, required_unless_present_any(["left", "right"]), conflicts_with_all(&["left", "right"]))]
        both: Option<usize>,

        /// Extend intervals on the left side by the provided amount
        #[clap(short, long, required_unless_present_any(["both", "right"]))]
        left: Option<usize>,

        /// Extend intervals on the right side by the provided amount
        #[clap(short, long, required_unless_present_any(["both", "left"]))]
        right: Option<usize>,

        /// Genome file to validate extensions against
        #[clap(short, long)]
        genome: Option<String>,

        /// Allow for non-integer chromosome names
        #[clap(short = 'N', long)]
        named: bool,

        /// Format of input file
        #[clap(short = 'T', long, default_value = "bed3")]
        format: InputFormat,
    },

    /// Extracts FASTA sequences using intervals from a BED file
    GetFasta {
        /// BED file containing intervals to extract
        #[clap(short, long)]
        bed: Option<String>,

        /// FASTA file to extract sequences from (assumes <fasta>.fai exists)
        #[clap(short, long)]
        fasta: String,

        /// Output FASTA file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Format of input file
        #[clap(short = 'T', long, default_value = "bed3")]
        format: InputFormat,
    },

    /// Intersects two BED files
    Intersect {
        /// Input BED file to intersect (default=stdin)
        #[clap(short, long)]
        a: Option<String>,

        /// Secondary BED file to intersect
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

        /// Return the records from a that overlap with b instead of the intersection
        #[clap(short = 'q', long, conflicts_with = "with_target")]
        with_query: bool,

        /// Return the records from b that overlap with a instead of the intersection
        #[clap(short = 't', long, conflicts_with = "with_query")]
        with_target: bool,

        /// Only write the query record once if it overlaps with multiple target records
        #[clap(short, long, requires = "with_query", conflicts_with = "with_target")]
        unique: bool,

        /// Only report the intervals in the query that do not overlap with the target
        /// (i.e. the inverse of the intersection)
        #[clap(short = 'v', long, conflicts_with_all = &["with_query", "with_target", "unique"])]
        inverse: bool,

        /// Allow for non-integer chromosome names
        #[clap(short = 'N', long)]
        named: bool,

        /// Stream the input files instead of loading them into memory
        /// (only works if both files are sorted)
        #[clap(short = 'S', long, conflicts_with_all = &["with_query", "with_target", "unique", "inverse"])]
        stream: bool,

        /// Input formats
        #[clap(short = 'T', long, default_value = "bed3")]
        format: InputFormat,
    },

    /// Merges intervals of a BED file with overlapping regions
    Merge {
        /// Input BED file to merge (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Assume input is sorted (default=false)
        #[clap(short, long)]
        sorted: bool,

        /// Allow for non-integer chromosome names
        #[clap(short = 'N', long)]
        named: bool,

        /// Stream the input file instead of loading it into memory
        ///
        /// Note that this requires the input file to be sorted
        /// and will result in undefined behavior if it is not.
        ///
        /// Currently does not support non-integer chromosome names.
        #[clap(short = 'S', long, conflicts_with = "named")]
        stream: bool,

        /// Set the format of the input bed
        #[clap(short = 'T', long, default_value = "bed3")]
        format: InputFormat,
    },

    /// Builds a two column map of chromosome names to integers
    /// and writes the map and BED file with integer chromosome names
    /// to disk
    ///
    /// The map file is a two column file with the first column
    /// containing the integer chromosome index and the second column
    /// containing the original chromosome name
    NameMap {
        /// Input BED file to map chromosome names (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Output map file to write to (default=name_map.tsv)
        #[clap(short, long)]
        map: Option<String>,
    },

    /// Generates a random BED file given some parameterizations
    Random {
        /// Number of intervals to generate (default = 10_000)
        #[clap(short, long, default_value = "10000")]
        n_intervals: usize,

        /// Length of intervals to generate (default = 150)
        #[clap(short, long, default_value = "150")]
        l_intervals: usize,

        /// Number of chromosomes to generate (default = 23)
        #[clap(short = 'c', long, default_value = "23")]
        n_chr: usize,

        /// Maximum length of chromosomes (default = 250_000_000)
        #[clap(short, long, default_value = "250000000")]
        max_chr_len: usize,

        /// Seed to use for random number generation (no default)
        #[clap(short, long)]
        seed: Option<usize>,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Genome file to set boundaries for random intervals
        #[clap(short, long, conflicts_with_all = &["max_chr_len", "n_chr"])]
        genome: Option<String>,

        /// Allow for non-integer chromosome names in genome file + output
        #[clap(short = 'N', long)]
        named: bool,

        /// Set the output format
        #[clap(short = 'T', long, default_value = "bed3")]
        format: InputFormat,
    },

    /// Randomly sample a BED file
    Sample {
        /// Input BED file to sample (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Number of intervals to sample (choose one of n or f)
        #[clap(short, long, required_unless_present_any(&["fraction"]), conflicts_with_all(&["fraction"]))]
        number: Option<usize>,

        /// Fraction of intervals to sample (choose one of n or f)
        #[clap(short, long, required_unless_present_any(&["number"]), conflicts_with_all(&["number"]))]
        fraction: Option<f64>,

        /// Seed to use for random number generation (no default)
        #[clap(short, long)]
        seed: Option<usize>,

        /// Allow for non-integer chromosome names
        #[clap(short = 'N', long)]
        named: bool,

        /// Format of input file
        #[clap(short = 'T', long, default_value = "bed3")]
        format: InputFormat,
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

        /// Allow for non-integer chromosome names
        #[clap(short = 'N', long)]
        named: bool,

        /// Input file formats
        #[clap(short = 'T', long, default_value = "bed3")]
        format: InputFormat,
    },
}
