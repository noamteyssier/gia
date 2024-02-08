use super::Output;
use crate::types::InputFormat;
use clap::Parser;
use rand::{thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

#[derive(Parser, Debug, Clone)]
pub struct RandomArgs {
    #[clap(flatten)]
    pub output: Output,

    /// Number of intervals to generate (default = 10_000)
    #[clap(short, long, default_value = "10000")]
    pub n_intervals: usize,

    /// Length of intervals to generate (default = 150)
    #[clap(short, long, default_value = "150")]
    pub l_intervals: usize,

    /// Number of chromosomes to generate (default = 23)
    #[clap(short = 'c', long, default_value = "23")]
    pub n_chr: usize,

    /// Maximum length of chromosomes (default = 250_000_000)
    #[clap(short, long, default_value = "250000000")]
    pub max_chr_len: usize,

    /// Seed to use for random number generation (no default)
    #[clap(short, long)]
    pub seed: Option<usize>,

    /// Genome file to set boundaries for random intervals
    #[clap(short, long, conflicts_with_all = &["max_chr_len", "n_chr"])]
    pub genome: Option<String>,

    /// Allow for non-integer chromosome names in genome file + output
    #[clap(short = 'N', long)]
    pub named: bool,

    /// Set the output format
    #[clap(short = 'T', long, default_value = "bed3")]
    pub format: InputFormat,
}
impl RandomArgs {
    pub fn build_rng(&self) -> Box<dyn RngCore> {
        match self.seed {
            Some(seed) => Box::new(ChaChaRng::seed_from_u64(seed as u64)),
            None => Box::new(thread_rng()),
        }
    }
}
