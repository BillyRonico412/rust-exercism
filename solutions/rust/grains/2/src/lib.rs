pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2u64.pow(s - 1),
        _ => panic!(),
    }
}

pub fn total() -> u64 {
    (1..=64).fold(0, |acc, s| acc + square(s))
}
