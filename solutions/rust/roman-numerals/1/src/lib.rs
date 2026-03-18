use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let log_10 = (self.0 as f32).log10() as u32;
        let mut rest_number = self.0;
        let mut result = String::new();
        for exp in (0..=log_10).rev() {
            let current_number = rest_number / 10u32.pow(exp);
            let (tenth, fifth, oneth) = match exp {
                0 => ("X", "V", "I"),
                1 => ("C", "L", "X"),
                2 => ("M", "D", "C"),
                3 => ("M", "M", "M"),
                _ => unreachable!(),
            };
            let roman_term = match current_number {
                0 => "".to_owned(),
                1 => format!("{}", oneth),
                2 => format!("{}{}", oneth, oneth),
                3 => format!("{}{}{}", oneth, oneth, oneth),
                4 => format!("{}{}", oneth, fifth),
                5 => format!("{}", fifth),
                6 => format!("{}{}", fifth, oneth),
                7 => format!("{}{}{}", fifth, oneth, oneth),
                8 => format!("{}{}{}{}", fifth, oneth, oneth, oneth),
                9 => format!("{}{}", oneth, tenth),
                _ => unreachable!(),
            };
            result.push_str(roman_term.as_str());
            rest_number = self.0 % 10u32.pow(exp);
        }
        write!(f, "{}", result)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num)
    }
}
