use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();
    (1..sum).for_each(|a| {
        ((a + 1)..(sum - a)).for_each(|b: u32| {
            let c = sum - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                set.insert([a, b, c]);
            }
        })
    });
    set
}
