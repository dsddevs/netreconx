use rayon::ThreadPool;

pub fn build_thread_pool() -> Result<ThreadPool, anyhow::Error> {
    let num_cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus * 2) // Reasonable multiplier for I/O bound tasks
        .build()?;
    Ok(pool)
}