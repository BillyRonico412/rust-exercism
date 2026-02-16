#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///

fn convert_into_decimal(number: &[u32], from_base: u32) -> u32 {
    number
        .iter()
        .copied()
        .enumerate()
        .map(|(i, digit)| digit * from_base.pow((number.len() - i - 1) as u32))
        .sum()
}

fn convert_from_decimal(number: u32, to_base: u32) -> Vec<u32> {
    if number < to_base {
        return vec![number];
    }
    let mut nb_digits = 0;
    let mut res = Vec::new();
    while number > to_base.pow(nb_digits) {
        nb_digits += 1;
    }
    let mut number = number;
    for rank_digit in (0..nb_digits).rev() {
        let factor = to_base.pow(rank_digit);
        res.push(number / factor);
        number %= factor;
    }
    res
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        Err(Error::InvalidInputBase)
    } else if number.iter().any(|&n| n >= from_base) {
        Err(Error::InvalidDigit(from_base))
    } else if to_base <= 1 {
        Err(Error::InvalidOutputBase)
    } else if number.len() == 0 {
        Ok(vec![0])
    } else {
        Ok(convert_from_decimal(
            convert_into_decimal(number, from_base),
            to_base,
        ))
    }
}
