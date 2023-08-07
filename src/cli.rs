use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
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

    /// Intersects two BED files
    Intersect {
        #[clap(short, long)]
        a: Option<String>,

        #[clap(short, long)]
        b: String,

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

    /// Sorts a BED file by chromosome, start, and end
    Sort {
        /// Input GIA file to sort (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output GIA file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,
    },
}
