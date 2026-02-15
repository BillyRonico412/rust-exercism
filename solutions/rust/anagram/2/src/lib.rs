use std::collections::{HashMap, HashSet};

fn get_nb_chars(word: &str) -> HashMap<char, u32> {
    let mut nb_chars = HashMap::new();
    for c in word.chars() {
        let cpt = nb_chars.entry(c).or_insert(0);
        *cpt += 1;
    }
    nb_chars
}

pub fn anagrams_for<'a>(source_word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let source_word = source_word.to_lowercase();
    let nb_chars_source = get_nb_chars(&source_word);
    possible_anagrams
        .iter()
        .filter(|&&test_word| {
            if source_word.len() != test_word.len() {
                return false;
            }
            let test_word = test_word.to_lowercase();
            source_word != test_word && nb_chars_source == get_nb_chars(&test_word)
        })
        .copied()
        .collect()
}
