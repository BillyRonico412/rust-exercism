use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..sum)
        .flat_map(|a| {
            ((a + 1)..(sum - a)).filter_map(move |b: u32| {
                let c = sum - a - b;
                (a.pow(2) + b.pow(2) == c.pow(2)).then_some([a, b, c])
            })
        })
        .collect()
}
