fn encode_thousand(n: u64, term: &str, divider: u64) -> String {
    if n.is_multiple_of(divider) {
        format!("{} {}", encode_recursive(n / divider), term)
    } else {
        format!(
            "{} {} {}",
            encode_recursive(n / divider),
            term,
            encode_recursive(n % divider)
        )
    }
}

fn encode_ten(term: &str, u: u64) -> String {
    if u == 0 {
        term.to_string()
    } else {
        format!("{}-{}", term, encode_recursive(u))
    }
}

fn encode_recursive(n: u64) -> String {
    match n {
        0..20 => String::from(match n {
            0 => "",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => unreachable!(),
        }),
        20..100 => {
            let u = n % 10;
            match n / 10 {
                2 => encode_ten("twenty", u),
                3 => encode_ten("thirty", u),
                4 => encode_ten("forty", u),
                5 => encode_ten("fifty", u),
                6 => encode_ten("sixty", u),
                7 => encode_ten("seventy", u),
                8 => encode_ten("eighty", u),
                9 => encode_ten("ninety", u),
                _ => unreachable!(),
            }
        }
        100..1_000 => encode_thousand(n, "hundred", 100),
        1_000..1_000_000 => encode_thousand(n, "thousand", 1_000),
        1_000_000..1_000_000_000 => encode_thousand(n, "million", 1_000_000),
        1_000_000_000..1_000_000_000_000 => encode_thousand(n, "billion", 1_000_000_000),
        1_000_000_000_000..1_000_000_000_000_000 => {
            encode_thousand(n, "trillion", 1_000_000_000_000)
        }
        1_000_000_000_000_000..1_000_000_000_000_000_000 => {
            encode_thousand(n, "quadrillion", 1_000_000_000_000_000)
        }
        1_000_000_000_000_000_000..=u64::MAX => {
            encode_thousand(n, "quintillion", 1_000_000_000_000_000_000)
        }
    }
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        String::from("zero")
    } else {
        encode_recursive(n)
    }
}
