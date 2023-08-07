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
