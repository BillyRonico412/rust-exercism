pub fn encode(source: &str) -> String {
    let mut nb_chars = 0;
    let mut last_char_option = None;
    let mut result = String::new();
    for c in source.chars() {
        if let Some(last_char) = last_char_option
            && c != last_char
        {
            if nb_chars == 1 {
                result = format!("{}{}", result, last_char);
            } else {
                result = format!("{}{}{}", result, nb_chars, last_char);
            }
            nb_chars = 0;
        }
        last_char_option = Some(c);
        nb_chars += 1;
    }
    if let Some(last_char) = last_char_option {
        if nb_chars == 1 {
            result = format!("{}{}", result, last_char);
        } else {
            result = format!("{}{}{}", result, nb_chars, last_char);
        }
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
