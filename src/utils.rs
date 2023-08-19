use rand::{thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

pub fn build_rng(seed: Option<usize>) -> Box<dyn RngCore> {
    match seed {
        Some(seed) => Box::new(ChaChaRng::seed_from_u64(seed as u64)),
        None => Box::new(thread_rng()),
    }
}
