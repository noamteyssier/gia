use anyhow::Result;

pub fn setup_rayon(threads: Option<usize>) -> Result<()> {
    let num_threads = match threads {
        Some(threads) => threads,
        None => 1,
    };
    // rayon defaults to using all available threads
    // so no need to set it if the user didn't specify
    if num_threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()?;
    }
    Ok(())
}
