use std::collections::BTreeMap;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut marked_vec = (2..=upper_bound)
        .map(|n| (n, false))
        .collect::<BTreeMap<_, _>>();
    for cursor_1 in 2..=upper_bound {
        for cursor_2 in (cursor_1 + 1)..=upper_bound {
            if marked_vec[&cursor_2] {
                continue;
            }
            if cursor_2 % cursor_1 == 0 {
                marked_vec.insert(cursor_2, true);
            }
        }
    }
    return marked_vec
        .iter()
        .filter_map(|(&n, &b)| if !b { Some(n) } else { None })
        .collect();
}
