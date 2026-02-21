use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut map = HashSet::with_capacity((b'z' - b'a' + 1).into());
    candidate
        .bytes()
        .map(|b| b.to_ascii_lowercase())
        .all(|b| b.is_ascii_whitespace() || b == b'-' || map.insert(b))
}
