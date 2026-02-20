/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut count = 0;
    let mut sum = 0;
    for c in isbn.chars() {
        if c == '-' {
            continue;
        }
        match (count, c) {
            (9, c) if !c.is_ascii_digit() && c != 'X' => return false,
            (9, c) => sum += c.to_digit(10).unwrap_or(10),
            (_, c) if !c.is_ascii_digit() => return false,
            _ => sum += c.to_digit(10).unwrap() * (10 - count),
        }
        count += 1
    }
    count == 10 && sum % 11 == 0
}
