/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn
        .chars()
        .any(|c| !c.is_ascii_digit() && c != '-' && c != 'X')
    {
        return false;
    }
    let isbn_chars = isbn.chars().filter(|&c| c != '-').collect::<Vec<char>>();

    isbn_chars.len() == 10
        && isbn_chars.iter().enumerate().all(|(i, &c)| match (i, c) {
            (9, c) => c == 'X' || c.is_ascii_digit(),
            _ => c.is_ascii_digit(),
        })
        && isbn_chars.iter().enumerate().fold(0, |acc, (i, &c)| {
            acc + match (i, c) {
                (9, 'X') => 10,
                _ => (10 - i as u32) * c.to_digit(10).unwrap(),
            }
        }) % 11
            == 0
}
