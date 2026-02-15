pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '{' | '[' | '(' => stack.push(c),
            '}' | ']' | ')' => {
                if let Some(last_open) = stack.last() {
                    match (last_open, c) {
                        ('{', '}') | ('[', ']') | ('(', ')') => stack.pop(),
                        _ => return false,
                    }
                } else {
                    return false;
                };
            }
            _ => continue,
        }
    }
    stack.len() == 0
}
