pub fn encode(source: &str) -> String {
    let mut nb_chars = 0;
    let mut result = String::new();
    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        nb_chars += 1;
        if chars.peek() == Some(&c) {
            continue;
        }
        if nb_chars > 1 {
            result.push_str(&nb_chars.to_string());
        }
        result.push(c);
        nb_chars = 0;
    }
    result
}

pub fn decode(source: &str) -> String {
    let mut nb_chars = String::new();
    let mut result = String::new();
    for c in source.chars() {
        if c.is_ascii_digit() {
            nb_chars.push(c);
            continue;
        }
        let nb = nb_chars.parse::<u32>().unwrap_or(1);
        for _ in 0..nb {
            result.push(c);
        }
        nb_chars = String::new();
    }
    result
}
