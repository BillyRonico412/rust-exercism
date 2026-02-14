/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.trim();
    if code.len() <= 1 {
        return false;
    }
    let mut sum = 0;
    let mut is_double = false;
    for c in code.trim().chars().rev() {
        if c == ' ' {
            continue;
        }
        match c.to_digit(10) {
            None => return false,
            Some(d) => {
                if !is_double {
                    sum += d
                } else if d * 2 < 9 {
                    sum += d * 2
                } else {
                    sum += d * 2 - 9
                }
                is_double = !is_double;
            }
        }
    }
    sum % 10 == 0
}
