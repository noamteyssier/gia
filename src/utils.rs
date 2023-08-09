use anyhow::Result;
use rand::{thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

pub fn setup_rayon(threads: Option<usize>) -> Result<()> {
    let num_threads = if let Some(t) = threads { t } else { 1 };
    // rayon defaults to using all available threads
    // so no need to set it if the user didn't specify
    if num_threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()?;
    }
    Ok(())
}

pub fn build_rng(seed: Option<usize>) -> Box<dyn RngCore> {
    match seed {
        Some(seed) => Box::new(ChaChaRng::seed_from_u64(seed as u64)),
        None => Box::new(thread_rng()),
    }
}
