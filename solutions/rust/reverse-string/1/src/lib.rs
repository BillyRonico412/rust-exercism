pub fn reverse(input: &str) -> String {
    input.chars().rev().fold(String::from(""), |mut acc, cur| {
        acc.push(cur);
        acc
    })
}
