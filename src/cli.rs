use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
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
    },

    /// Sorts a BED file by chromosome, start, and end
    Sort {
        /// Input GIA file to sort (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output GIA file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,
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

        /// Name map file to use (in case chromosome names are non-integers)
        #[clap(short, long)]
        map: Option<String>,

        /// Number of threads to use (use zero for all available cores)
        #[clap(short, long, default_value = "1")]
        threads: Option<usize>,
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
}
