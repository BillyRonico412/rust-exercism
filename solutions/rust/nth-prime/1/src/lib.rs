fn is_prime(x: u32) -> bool {
    (2..x).all(|d| x % d != 0)
}

pub fn nth(n: u32) -> u32 {
    let mut x = 2;
    for _ in 0..n {
        loop {
            x += 1;
            if is_prime(x) {
                break;
            }
        }
    }
    x
}
