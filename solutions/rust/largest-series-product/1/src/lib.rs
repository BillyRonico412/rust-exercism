#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if string_digits.len() < span {
        Err(Error::SpanTooLong)
    } else if let Some(invalid_digit) = string_digits.chars().find(|c| !c.is_numeric()) {
        Err(Error::InvalidDigit(invalid_digit))
    } else if string_digits.len() == 0 || span == 0 {
        Ok(1)
    } else {
        Ok(string_digits
            .chars()
            .collect::<Vec<_>>()
            .windows(span)
            .map(|chunck| {
                chunck
                    .iter()
                    .fold(1u64, |acc, c| acc * c.to_string().parse::<u64>().unwrap())
            })
            .max()
            .unwrap())
    }
}
