use std::ops::{Add, Mul, Sub};

use itertools::{EitherOrBoth, Itertools};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
enum Sign {
    Negative,
    Positive,
}

enum Part {
    Integer,
    Decimal,
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
struct Digits(Vec<u8>);

impl Digits {
    fn shorten(&self, part: Part) -> Self {
        match part {
            Part::Integer => Self(self.0.iter().skip_while(|&&c| c == 0).copied().collect()),
            Part::Decimal => {
                let mut shorted = self
                    .0
                    .iter()
                    .rev()
                    .skip_while(|&&c| c == 0)
                    .copied()
                    .collect::<Vec<_>>();
                shorted.reverse();
                Self(shorted)
            }
        }
    }

    fn align(&self, b: &Self, part: &Part) -> Self {
        match part {
            Part::Integer => {
                let mut a = self.clone();
                if a.0.len() == b.0.len() {
                    return a;
                }
                a.0.reverse();
                while a.0.len() < b.0.len() {
                    a.0.push(0);
                }
                a.0.reverse();
                a
            }
            Part::Decimal => {
                let mut a = self.clone();
                while a.0.len() < b.0.len() {
                    a.0.push(0);
                }
                a
            }
        }
    }

    fn align_double(a: &Self, b: &Self, part: &Part) -> (Self, Self) {
        (a.align(b, part), b.align(a, part))
    }

    fn to_decimal(&self, offset_right: usize, sign: Sign) -> Decimal {
        let offset_left = self.0.len() - offset_right;
        let mut decimal = Decimal {
            sign,
            integer_part: Digits(self.0.iter().take(offset_left).copied().collect()),
            decimal_part: Digits(self.0.iter().skip(offset_left).copied().collect()),
        };
        decimal.shorten();
        if decimal.integer_part.0.is_empty() && decimal.decimal_part.0.is_empty() {
            decimal.sign = Sign::Positive;
        }
        decimal
    }
}

impl Add for Digits {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.0.reverse();
        rhs.0.reverse();
        let mut retain = 0;
        let mut result = vec![];
        self.0.iter().zip_longest(rhs.0.iter()).for_each(|duo| {
            let (a, b) = match duo {
                EitherOrBoth::Both(a, b) => (*a, *b),
                EitherOrBoth::Left(a) => (*a, 0),
                EitherOrBoth::Right(b) => (0, *b),
            };
            let sum = a + b + retain;
            let new_digit = sum % 10;
            retain = sum / 10;
            result.push(new_digit);
        });
        if retain > 0 {
            result.push(retain);
        }
        result.reverse();
        Digits(result)
    }
}

impl Sub for Digits {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        self.0.reverse();
        rhs.0.reverse();
        let mut retain = 0;
        let mut result = vec![];
        self.0.iter().zip(rhs.0.iter()).for_each(|(&a, &b)| {
            let sub;
            if a >= (b + retain) {
                sub = a - b - retain;
                retain = 0;
            } else {
                sub = a + 10 - b - retain;
                retain = 1;
            }
            result.push(sub);
        });
        result.reverse();
        Digits(result)
    }
}

impl Mul for Digits {
    type Output = Self;

    fn mul(self, mut rhs: Self) -> Self::Output {
        rhs.0.reverse();
        rhs.0
            .iter()
            .enumerate()
            .fold(Digits(vec![]), |mut acc1, (index, &digit)| {
                let mut m = (0..digit).fold(Digits(vec![]), |mut acc2, _| {
                    acc2 = acc2 + self.clone();
                    acc2
                });
                (0..index).for_each(|_| {
                    m.0.push(0);
                });
                acc1 = acc1 + m;
                acc1
            })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Decimal {
    sign: Sign,
    integer_part: Digits,
    decimal_part: Digits,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut split = input.split(".");

        let (integer_part_str, decimal_part_str) = match (split.next(), split.next(), split.next())
        {
            (Some(integer_part_str), None, None) => (integer_part_str, ""),
            (Some(integer_part_str), Some(decimal_part_str), None) => {
                (integer_part_str, decimal_part_str)
            }
            _ => return None,
        };

        let explicit_sign = integer_part_str.chars().next().and_then(|c| match c {
            '+' => Some(Sign::Positive),
            '-' => Some(Sign::Negative),
            _ => None,
        });

        let integer_part = integer_part_str
            .chars()
            .skip(if explicit_sign.is_some() { 1 } else { 0 })
            .map(|c| c.to_digit(10).map(|it| it as u8))
            .collect::<Option<Vec<u8>>>()?;

        let decimal_part = decimal_part_str
            .chars()
            .map(|c| c.to_digit(10).map(|it| it as u8))
            .collect::<Option<Vec<u8>>>()?;

        let mut decimal = Self {
            sign: explicit_sign.unwrap_or(Sign::Positive),
            integer_part: Digits(integer_part),
            decimal_part: Digits(decimal_part),
        };
        decimal.shorten();

        Some(decimal)
    }

    fn shorten(&mut self) {
        self.integer_part = self.integer_part.shorten(Part::Integer);
        self.decimal_part = self.decimal_part.shorten(Part::Decimal);
    }

    fn align(decimal_1: &Decimal, decimal_2: &Decimal) -> (Decimal, Decimal) {
        let (integer_part_1, integer_part_2) = Digits::align_double(
            &decimal_1.integer_part,
            &decimal_2.integer_part,
            &Part::Integer,
        );

        let (decimal_part_1, decimal_part_2) = Digits::align_double(
            &decimal_1.decimal_part,
            &decimal_2.decimal_part,
            &Part::Decimal,
        );

        (
            Decimal {
                sign: decimal_1.sign.clone(),
                integer_part: integer_part_1,
                decimal_part: decimal_part_1,
            },
            Decimal {
                sign: decimal_2.sign.clone(),
                integer_part: integer_part_2,
                decimal_part: decimal_part_2,
            },
        )
    }

    fn concat(&self) -> Digits {
        Digits(
            self.integer_part
                .0
                .iter()
                .chain(self.decimal_part.0.iter())
                .copied()
                .collect(),
        )
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (a, b) = Decimal::align(self, other);
        match a.sign.partial_cmp(&b.sign) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        let ord = 'ord: {
            match a.integer_part.partial_cmp(&b.integer_part) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => break 'ord ord,
            }
            a.decimal_part.partial_cmp(&b.decimal_part)
        };
        match self.sign {
            Sign::Positive => ord,
            Sign::Negative => ord.map(|ord| ord.reverse()),
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self.sign, &rhs.sign) {
            (Sign::Positive, Sign::Negative) => {
                let mut rhs = rhs;
                rhs.sign = Sign::Positive;
                return self - rhs;
            }
            (Sign::Negative, Sign::Positive) => {
                let mut this = self;
                this.sign = Sign::Positive;
                return rhs - this;
            }
            _ => {}
        }
        let (d1, d2) = Decimal::align(&self, &rhs);
        let i1 = d1.concat();
        let i2 = d2.concat();
        let add = i1 + i2;
        add.to_decimal(d1.decimal_part.0.len(), d1.sign)
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self.sign, &rhs.sign) {
            (Sign::Positive, Sign::Negative) => {
                let mut rhs = rhs;
                rhs.sign = Sign::Positive;
                return self + rhs;
            }
            (Sign::Negative, Sign::Positive) => {
                let mut rhs = rhs;
                rhs.sign = Sign::Negative;
                return self + rhs;
            }
            _ => {}
        }

        let (d1, d2, sign) = match (&self.sign, self > rhs) {
            (Sign::Positive, true) => (self, rhs, Sign::Positive),
            (Sign::Positive, false) => (rhs, self, Sign::Negative),
            (Sign::Negative, true) => (rhs, self, Sign::Positive),
            (Sign::Negative, false) => (self, rhs, Sign::Negative),
        };

        let (d1, d2) = Self::align(&d1, &d2);
        let i1 = d1.concat();
        let i2 = d2.concat();
        let sub = i1 - i2;
        sub.to_decimal(d1.decimal_part.0.len(), sign)
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let sign = if self.sign == rhs.sign {
            Sign::Positive
        } else {
            Sign::Negative
        };

        let i1 = self.concat();
        let i2 = rhs.concat();
        let mul = i1 * i2;
        mul.to_decimal(self.decimal_part.0.len() + rhs.decimal_part.0.len(), sign)
    }
}
