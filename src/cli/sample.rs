use super::{Output, SingleInput};
use clap::Parser;
use rand::{thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

#[derive(Parser, Debug)]
pub struct SampleArgs {
    #[clap(flatten)]
    pub input: SingleInput,

    #[clap(flatten)]
    pub params: SampleParams,

    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(next_help_heading = "Parameters")]
pub struct SampleParams {
    /// Number of intervals to sample (choose one of n or f)
    #[clap(short, long, required_unless_present_any(&["fraction"]), conflicts_with_all(&["fraction"]))]
    pub number: Option<usize>,

    /// Fraction of intervals to sample (choose one of n or f)
    #[clap(short, long, required_unless_present_any(&["number"]), conflicts_with_all(&["number"]))]
    pub fraction: Option<f64>,

    /// Seed to use for random number generation (no default)
    #[clap(short, long)]
    pub seed: Option<usize>,
}
impl SampleParams {
    pub fn build_rng(&self) -> Box<dyn RngCore> {
        match self.seed {
            Some(seed) => Box::new(ChaChaRng::seed_from_u64(seed as u64)),
            None => Box::new(thread_rng()),
        }
    }
}
