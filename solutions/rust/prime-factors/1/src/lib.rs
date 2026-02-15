pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut d = 2;
    let mut vec = Vec::new();
    while n != 1 {
        if n % d != 0 {
            d += 1;
            continue;
        }
        vec.push(d);
        n /= d;
    }
    vec
}
