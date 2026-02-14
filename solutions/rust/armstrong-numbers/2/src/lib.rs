pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let len = num_string.len() as u32;
    num_string
        .chars()
        .try_fold(0, |acc, c| c.to_digit(10).map(|d| acc + d.pow(len)))
        .map_or(false, |sum| sum == num)
}
