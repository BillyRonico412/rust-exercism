const DIGIT_TO_STRING: [&str; 11] = [
    "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start_bottles: usize, take_down: usize) -> String {
    ((start_bottles - take_down + 1)..=start_bottles)
        .rev()
        .map(|n| {
            let bottle_plural = |n| if n == 1 { "bottle" } else { "bottles" };
            let bottle_before = bottle_plural(n);
            let bottle_after = bottle_plural(n - 1);
            format!(
                "{0} green {bottle_before} hanging on the wall,\n\
                {0} green {bottle_before} hanging on the wall,\n\
                And if one green bottle should accidentally fall,\n\
                There'll be {1} green {bottle_after} hanging on the wall.\n\n",
                DIGIT_TO_STRING[n],
                DIGIT_TO_STRING[n - 1].to_lowercase(),
            )
        })
        .collect()
}
