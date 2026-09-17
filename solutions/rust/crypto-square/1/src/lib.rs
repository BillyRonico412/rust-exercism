pub fn encrypt(input: &str) -> String {
    let normalized_input = input
        .chars()
        .filter_map(|c| c.is_alphanumeric().then_some(c.to_ascii_lowercase()))
        .collect::<Vec<_>>();

    let len = normalized_input.len();

    let mut c = 0;
    while (c == 0 || c * (c - 1) < len) && c * c < len {
        c += 1;
    }

    (0..c)
        .map(|i| {
            normalized_input
                .chunks(c)
                .map(|chunck| chunck.get(i).unwrap_or(&' '))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
