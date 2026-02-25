fn encode_thousand(n: u64, term: &str, divider: u64) -> String {
    if n % divider == 0 {
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
    let nb_digits = (n as f64).log10() as u64 + 1;
    match nb_digits {
        1 => String::from(match n {
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
            _ => unreachable!(),
        }),
        2 => match (n / 10, n % 10) {
            (1, _) => String::from(match n {
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
            (d, u) => match d {
                2 => encode_ten("twenty", u),
                3 => encode_ten("thirty", u),
                4 => encode_ten("forty", u),
                5 => encode_ten("fifty", u),
                6 => encode_ten("sixty", u),
                7 => encode_ten("seventy", u),
                8 => encode_ten("eighty", u),
                9 => encode_ten("ninety", u),
                _ => unreachable!(),
            },
        },
        3 => encode_thousand(n, "hundred", 100),
        4..=6 => encode_thousand(n, "thousand", 1_000),
        7..=9 => encode_thousand(n, "million", 1_000_000),
        10..=12 => encode_thousand(n, "billion", 1_000_000_000),
        13..=15 => encode_thousand(n, "trillion", 1_000_000_000_000),
        16..=18 => encode_thousand(n, "quadrillion", 1_000_000_000_000_000),
        19..=21 => encode_thousand(n, "quintillion", 1_000_000_000_000_000_000),
        _ => unreachable!(),
    }
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        String::from("zero")
    } else {
        encode_recursive(n)
    }
}
