pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    num_string
        .chars()
        .try_fold(0, |acc, c| {
            c.to_digit(10).map(|d| acc + d.pow(num_string.len() as u32))
        })
        .map_or(false, |sum| sum == num)
}
