pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        _ => {
            let start = list
                .windows(2)
                .map(|list| format!("For want of a {} the {} was lost.\n", list[0], list[1]))
                .collect::<String>();
            let end = format!("And all for the want of a {}.", list[0]);
            format!("{}{}", start, end)
        }
    }
}
