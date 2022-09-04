use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap()
        .install(|| {
            input
                .join("")
                .par_chars()
                .filter(|c| c.is_alphabetic())
                .fold(
                    || HashMap::new(),
                    |mut acc: HashMap<char, usize>, c| {
                        for c in c.to_lowercase() {
                            *acc.entry(c).or_insert(0) += 1;
                        }
                        acc
                    },
                )
                .reduce(
                    || HashMap::new(),
                    |mut left, right| {
                        for (c, count) in right.iter() {
                            *left.entry(*c).or_insert(0) += count;
                        }
                        left
                    },
                )
        })
}
