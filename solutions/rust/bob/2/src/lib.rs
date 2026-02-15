pub fn reply(message: &str) -> &str {
    let message_trim = message.trim();
    if message_trim.is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = message_trim.ends_with('?');
    let is_yell = message_trim.chars().any(|c| c.is_ascii_alphabetic())
        && message_trim
            .chars()
            .all(|c| !c.is_ascii_alphabetic() || c.is_uppercase());
    match (is_question, is_yell) {
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (true, true) => "Calm down, I know what I'm doing!",
        _ => "Whatever.",
    }
}
