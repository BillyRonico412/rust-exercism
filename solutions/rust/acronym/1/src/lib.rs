pub fn abbreviate(phrase: &str) -> String {
    phrase
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
        .filter_map(|(i, window)| {
            if let (Some(c1), Some(c2)) = (window.get(0), window.get(1)) {
                match (i, c1, c2) {
                    (0, _, _) => Some(c1),
                    (_, c1, c2) if !c1.is_alphabetic() && *c1 != '\'' && c2.is_alphabetic() => {
                        Some(c2)
                    }
                    (_, c1, c2) if c1.is_ascii_lowercase() && c2.is_ascii_uppercase() => Some(c2),
                    _ => None,
                }
            } else {
                None
            }
        })
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
