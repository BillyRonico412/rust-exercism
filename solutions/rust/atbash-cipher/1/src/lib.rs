/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(|c| {
            if c.is_numeric() {
                Some(c)
            } else if c.is_alphabetic() {
                let c = c.to_ascii_lowercase();
                let c = b'z' - (c as u8 - b'a');
                Some(c as char)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(|c| {
            if c.is_numeric() {
                Some(c)
            } else if c.is_alphabetic() {
                let c = b'a' + (b'z' - c as u8);
                Some(c as char)
            } else {
                None
            }
        })
        .collect()
}
