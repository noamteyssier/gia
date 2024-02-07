use crate::types::{Genome, Translater};
use anyhow::Result;
use bedrs::traits::IntervalBounds;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Growth {
    /// Amount to apply to function on both sides of intervals
    #[clap(short, long, required_unless_present_any(["left", "right"]), conflicts_with_all(&["left", "right"]))]
    pub both: Option<f64>,

    /// Amount to apply to function on the left side of intervals
    #[clap(short, long, required_unless_present_any(["both", "right"]))]
    pub left: Option<f64>,

    /// Amount to apply to function on the right side of intervals
    #[clap(short, long, required_unless_present_any(["both", "left"]))]
    pub right: Option<f64>,

    /// Convert values provided to percentages of the interval length
    #[clap(short, long)]
    pub percent: bool,

    /// Genome file to validate growth against
    #[clap(short, long)]
    pub genome: Option<String>,
}
impl Growth {
    pub fn get_values<I>(&self, iv: &I) -> (usize, usize)
    where
        I: IntervalBounds<usize, usize>,
    {
        if let Some(val) = self.both {
            self.calculate_percentage(iv, val, val)
        } else {
            self.calculate_percentage(
                iv,
                self.left.unwrap_or_default(),
                self.right.unwrap_or_default(),
            )
        }
    }

    fn calculate_percentage<I>(&self, iv: &I, val_left: f64, val_right: f64) -> (usize, usize)
    where
        I: IntervalBounds<usize, usize>,
    {
        if self.percent {
            (iv.f_len(val_left), iv.f_len(val_right))
        } else {
            (val_left as usize, val_right as usize)
        }
    }

    pub fn get_genome<'a>(&'a self, translater: Option<&'a Translater>) -> Result<Option<Genome>> {
        Genome::from_opt_path_immutable_with(self.genome.to_owned(), translater, false)
    }
}
