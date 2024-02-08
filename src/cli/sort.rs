use super::{Output, SingleInput};
use anyhow::Result;
use clap::Parser;
use rayon::ThreadPoolBuilder;

#[derive(Parser, Debug)]
pub struct SortArgs {
    #[clap(flatten)]
    pub input: SingleInput,
    #[clap(flatten)]
    pub output: Output,
    #[clap(flatten)]
    pub params: SortParams,
}

#[derive(Parser, Debug, Clone, Copy)]
pub struct SortParams {
    /// Number of threads to use for sorting (default=1)
    #[clap(short = 't', long, default_value = "1")]
    pub threads: usize,
}
impl SortParams {
    pub fn parallel(&self) -> bool {
        self.threads > 1
    }

    pub fn initialize_thread_pool(&self) -> Result<bool> {
        if self.threads > 1 {
            ThreadPoolBuilder::new()
                .num_threads(self.threads)
                .build_global()
                .unwrap();
            Ok(true)
        } else if self.threads == 0 {
            // by default, rayon uses all available cores
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
