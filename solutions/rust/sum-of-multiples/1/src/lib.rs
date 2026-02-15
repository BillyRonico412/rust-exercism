use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .copied()
        .filter(|f| *f > 0)
        .flat_map(|f| (f..limit).step_by(f as usize))
        .collect::<HashSet<u32>>()
        .iter()
        .sum()
}
