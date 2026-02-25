const SCRABBLE_SCORE_ARRAY: [(&str, u64); 7] = [
    ("AEIOULNRST", 1),
    ("DG", 2),
    ("BCMP", 3),
    ("FHVWY", 4),
    ("K", 5),
    ("JX", 8),
    ("QZ", 10),
];

pub fn score(word: &str) -> u64 {
    word.chars().fold(0, |acc, c| {
        if !c.is_ascii_alphabetic() {
            return acc;
        }
        SCRABBLE_SCORE_ARRAY
            .iter()
            .find_map(|&(seq, score)| {
                if !seq.contains(c.to_ascii_uppercase()) {
                    None
                } else {
                    Some(score)
                }
            })
            .unwrap_or(0)
    })
}
