use std::collections::{HashMap, HashSet};

fn get_nb_chars(word: &str) -> HashMap<char, i32> {
    let mut nb_chars = HashMap::new();
    for c in word.chars() {
        let c_lowercase = c.to_lowercase().next().unwrap_or(c);
        let cpt = nb_chars.entry(c_lowercase).or_insert(0);
        *cpt += 1;
    }
    nb_chars
}

fn is_anagram(source_word: &str, test_word: &str) -> bool {
    if source_word.len() != test_word.len() {
        return false;
    }
    let source_word = source_word.to_lowercase();
    let test_word = test_word.to_lowercase();
    if source_word == test_word {
        return false;
    }
    let nb_chars_source = get_nb_chars(&source_word);
    let nb_chars_test = get_nb_chars(&test_word);
    nb_chars_source == nb_chars_test
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res = HashSet::new();
    for test_word in possible_anagrams.iter() {
        let test_word = *test_word;
        if is_anagram(word, test_word) {
            res.insert(test_word);
        }
    }
    res
}
