pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!()
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).fold(0, |acc, s| acc + square(s))
}
