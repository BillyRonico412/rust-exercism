pub struct Luhn(bool);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.0
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        let is_valid = input
            .to_string()
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
            .map_or(false, |(sum, count)| count > 1 && sum % 10 == 0);
        Self(is_valid)
    }
}
