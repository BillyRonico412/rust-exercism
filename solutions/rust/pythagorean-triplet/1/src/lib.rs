use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..sum)
        .find_map(|a| {
            ((a + 1)..sum)
                .find_map(|b| {
                    ((b + 1)..sum)
                        .find(|&c| (a.pow(2) + b.pow(2) == c.pow(2)) && (a + b + c == sum))
                        .and_then(|c| Some((b, c)))
                })
                .and_then(|(b, c)| Some((a, b, c)))
        })
        .map_or_else(|| panic!(), |(a, b, c)| HashSet::from([[a, b, c]]))
}
