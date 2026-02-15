const DIVISEUR: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let result = DIVISEUR
        .iter()
        .filter_map(|(d, t)| if n % d != 0 { None } else { Some(t) })
        .copied()
        .collect::<String>();
    match result.len() {
        0 => n.to_string(),
        _ => result,
    }
}
