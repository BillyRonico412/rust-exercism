pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = message.trim().chars().last().is_some_and(|c| c == '?');
    let is_yell = message.trim().chars().any(|c| c.is_ascii_alphabetic())
        && message
            .trim()
            .chars()
            .all(|c| !c.is_ascii_alphabetic() || c.is_uppercase());
    match (is_question, is_yell) {
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (true, true) => "Calm down, I know what I'm doing!",
        _ => "Whatever.",
    }
}
