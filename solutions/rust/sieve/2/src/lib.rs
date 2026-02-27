pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut sieve_vec = vec![false; (upper_bound + 1) as usize];
    for cursor_1 in 2..upper_bound {
        if sieve_vec[cursor_1 as usize] {
            continue;
        }
        let mut sum = cursor_1 + cursor_1;
        while sum <= upper_bound {
            sieve_vec[sum as usize] = true;
            sum += cursor_1;
        }
    }
    sieve_vec
        .iter()
        .enumerate()
        .skip(2)
        .filter_map(|(n, &marked)| if marked { None } else { Some(n as u64) })
        .collect()
}
