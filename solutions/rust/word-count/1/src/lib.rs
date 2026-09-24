use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    let mut words = words.to_string();
    words.push(' ');

    let mut word = String::new();

    words.chars().for_each(|c| {
        if c.is_ascii_alphanumeric() || c == '\'' {
            word.push(c.to_ascii_lowercase());
        } else if word.len() != 0 {
            if word.starts_with('\'') {
                word = word.chars().skip(1).collect()
            }
            if word.ends_with('\'') {
                word = word.chars().take(word.len() - 1).collect()
            }
            if word.len() == 0 {
                return;
            }
            *map.entry(word.clone()).or_insert(0) += 1;
            word = String::new();
        }
    });
    map
}
