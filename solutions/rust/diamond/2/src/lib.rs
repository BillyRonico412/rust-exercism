fn get_diamond_line(c: char, index: usize, len: usize) -> String {
    let current_char = ((c as u8) - index as u8) as char;
    (0..len)
        .map(|i| {
            if i == index || i == len - index - 1 {
                current_char
            } else {
                ' '
            }
        })
        .collect()
}

pub fn get_diamond(c: char) -> Vec<String> {
    let diff = ((c as u8) - b'A') as usize;
    let len = diff * 2 + 1;
    (1..=diff)
        .rev()
        .chain([0])
        .chain(1..=diff)
        .map(|index| get_diamond_line(c, index, len))
        .collect()
}
