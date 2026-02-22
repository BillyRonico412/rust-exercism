use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }

    fn new(x: u64, y: u64) -> Self {
        Palindrome {
            value: x * y,
            factors: HashSet::from([(x, y)]),
        }
    }

    fn is_palindrome(value: u64) -> bool {
        let product_to_string = value.to_string();
        let len = product_to_string.len();
        let (left, right) = if len % 2 == 0 {
            (
                &product_to_string[..(len / 2)],
                &product_to_string[(len / 2)..],
            )
        } else {
            (
                &product_to_string[..(len / 2)],
                &product_to_string[(len / 2 + 1)..],
            )
        };
        left == right
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut res: Option<(Palindrome, Palindrome)> = None;
    for x in min..=max {
        for y in x..=max {
            let value = x * y;
            if !Palindrome::is_palindrome(value) {
                continue;
            }
            match &mut res {
                None => {
                    res = Some((Palindrome::new(x, y), Palindrome::new(x, y)));
                }
                Some((smallest_pal, largest_pal)) => {
                    if smallest_pal.value == value {
                        smallest_pal.factors.insert((x, y));
                    } else if smallest_pal.value > value {
                        *smallest_pal = Palindrome::new(x, y);
                    }

                    if largest_pal.value == value {
                        largest_pal.factors.insert((x, y));
                    } else if largest_pal.value < value {
                        *largest_pal = Palindrome::new(x, y)
                    }
                }
            }
        }
    }
    res
}
