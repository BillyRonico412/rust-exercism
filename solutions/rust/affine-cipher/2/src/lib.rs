const ALPHABET_SIZE: i32 = 26;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, ALPHABET_SIZE) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plaintext
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                let c = c.to_ascii_lowercase();
                let x = (c as u8) - b'a';
                let e_x = (a * x as i32 + b) % ALPHABET_SIZE;
                let e_x = ((e_x + ALPHABET_SIZE).rem_euclid(ALPHABET_SIZE)) as u8;
                let c: char = (e_x + b'a') as char;
                Some(c)
            } else if c.is_numeric() {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunck| chunck.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" "))
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, ALPHABET_SIZE) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let mmi = get_mmi(a);
    Ok(ciphertext
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                let c = (c as u8 - b'a') as i32;
                let c = ((mmi * (c - b)).rem_euclid(26)) as u8;
                let c = (b'a' + c) as char;
                Some(c)
            } else if c.is_numeric() {
                Some(c)
            } else {
                None
            }
        })
        .collect())
}

fn pgcd(x: u32, y: u32) -> u32 {
    let x_mod_y = x % y;
    if x_mod_y == 0 { y } else { pgcd(y, x_mod_y) }
}

fn is_coprime(x: i32, y: i32) -> bool {
    let x = x.abs() as u32;
    let y = y.abs() as u32;
    let (x, y) = if x > y { (x, y) } else { (y, x) };
    pgcd(x, y) == 1
}

fn get_mmi(a: i32) -> i32 {
    let mut k = 1;
    while (ALPHABET_SIZE * k + 1) % a != 0 {
        k += 1;
    }
    (ALPHABET_SIZE * k + 1) / a
}
