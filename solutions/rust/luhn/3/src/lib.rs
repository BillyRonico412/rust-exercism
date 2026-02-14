pub fn is_valid(code: &str) -> bool {
    code.trim()
        .chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), c| {
            c.to_digit(10).map(|d| match (count % 2 == 1, d * 2 < 9) {
                (false, _) => (sum + d, count + 1),
                (true, true) => (sum + d * 2, count + 1),
                (true, false) => (sum + d * 2 - 9, count + 1),
            })
        })
        .map_or(false, |(sum, count)| count > 1 && sum % 10 == 0)
}
