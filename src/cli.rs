use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Merges intervals of a BED file with overlapping regions
    Merge {
        /// Input BED file to merge (default=stdin)
        #[clap(short, long)]
        input: Option<String>,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,
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
