use super::{Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ShiftArgs {
    #[clap(flatten)]
    pub input: SingleInput,
    #[clap(flatten)]
    pub params: ShiftParams,
    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug)]
#[clap(next_help_heading = "Parameters")]
pub struct ShiftParams {
    /// Path to genome file to use for bounds when shifting
    #[clap(short, long)]
    pub genome: Option<String>,

    /// Amount to shift intervals by (negative values shift to the left)
    #[clap(short, long, allow_hyphen_values = true)]
    pub amount: f64,

    /// Interprets the amount as a fraction of the interval length
    ///
    /// i.e. if the amount is 0.5, the interval will be shifted
    /// by half of its length. if the amount is 2, the interval
    /// will be shifted by twice its length.
    #[clap(short, long)]
    pub percent: bool,
}
impl ShiftParams {
    pub fn warn_args(&self) {
        if self.percent && self.amount < 1.0 {
            eprintln!("Warning: Provided shift amount is less than 1.0 and percent is not set. This will shift intervals by the rounded integer value, which may not be the intended behavior. If you want to shift by a percentage, set the percent flag.");
        }
    }
}
