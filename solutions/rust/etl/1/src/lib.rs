use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut res = BTreeMap::new();
    for (&score, letter_vec) in h {
        for &letter in letter_vec {
            res.insert(letter.to_ascii_lowercase(), score);
        }
    }
    res
}
