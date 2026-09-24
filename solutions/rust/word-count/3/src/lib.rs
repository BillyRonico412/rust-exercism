use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '\'')
        .filter_map(|word| {
            let word_trimed = word.trim_matches('\'');
            (word_trimed.len() > 0).then(|| word_trimed.to_lowercase())
        })
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word.to_string()).or_insert(0) += 1;
            acc
        })
}
