use rand::distr::{Distribution, Uniform};

enum CodedMode {
    Encode,
    Decode,
}

fn codec(mode: CodedMode, key: &str, s: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }
    s.char_indices()
        .map(|(i, c)| {
            c.is_ascii_lowercase()
                .then(|| {
                    let key_char = key.chars().nth(i % key.len())?;
                    let key_char_index = char_to_index(key_char);
                    let s_char_index = char_to_index(c);
                    let sign = match mode {
                        CodedMode::Encode => 1,
                        CodedMode::Decode => -1,
                    };
                    let cypher_char_index = ((s_char_index as i32 + (key_char_index as i32) * sign)
                        .rem_euclid(26)) as u8;
                    Some(index_to_char(cypher_char_index))
                })
                .flatten()
        })
        .collect()
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    codec(CodedMode::Encode, key, s)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    codec(CodedMode::Decode, key, s)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::rng();
    let distribution = Uniform::new(0u8, 26u8).unwrap();
    let key = (0..100)
        .map(|_| index_to_char(distribution.sample(&mut rng)))
        .collect::<String>();
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}

fn char_to_index(c: char) -> u8 {
    (c as u8) - b'a'
}

fn index_to_char(i: u8) -> char {
    ((i as u8) + b'a') as char
}
