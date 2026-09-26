use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap();

    pool.install(|| {
        input
            .par_iter()
            .fold(
                || HashMap::new(),
                |mut acc, s| {
                    s.trim()
                        .chars()
                        .filter(|c| c.is_alphabetic())
                        .for_each(|c| {
                            *acc.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                        });
                    acc
                },
            )
            .reduce(
                || HashMap::new(),
                |mut a, b| {
                    b.iter()
                        .for_each(|(&key, value)| *a.entry(key).or_insert(0) += value);
                    a
                },
            )
    })
}
