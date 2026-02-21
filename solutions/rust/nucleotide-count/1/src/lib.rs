use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna).and_then(|counts| counts.get(&nucleotide).copied().ok_or(nucleotide))
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .into_iter()
        .collect();
    for c in dna.chars() {
        match map.entry(c) {
            Entry::Vacant(_) => return Err(c),
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
        }
    }
    Ok(map)
}
