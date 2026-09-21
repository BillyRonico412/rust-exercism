pub fn number(user_number: &str) -> Option<String> {
    let phone_number = user_number
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();

    if phone_number.len() < 10 || phone_number.len() > 11 {
        return None;
    }

    let mut chars = phone_number.chars();
    if phone_number.len() == 11 && chars.next()? != '1' {
        return None;
    }

    let first_digit = chars.next()?;
    let fourth_digit = chars.skip(2).next()?;

    if !is_special_digit(first_digit) || !is_special_digit(fourth_digit) {
        return None;
    }

    if phone_number.len() == 11 {
        Some(phone_number.chars().skip(1).collect())
    } else {
        Some(phone_number)
    }
}

fn is_special_digit(c: char) -> bool {
    c.to_digit(10).is_some_and(|digit| 2 <= digit)
}
