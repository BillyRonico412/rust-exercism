use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(first) => list
            .windows(2)
            .map(|list| format!("For want of a {} the {} was lost.\n", list[0], list[1]))
            .chain(once(format!("And all for the want of a {first}")))
            .collect::<String>(),
    }
}
