pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(
            |c| match (c.is_ascii_alphabetic(), c.is_ascii_uppercase()) {
                (false, _) => c,
                (true, false) => (((c as u8 - b'a' + key) % 26) + b'a') as char,
                (true, true) => (((c as u8 - b'A' + key) % 26) + b'A') as char,
            },
        )
        .collect()
}
