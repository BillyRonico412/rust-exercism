use regex::Regex;
use std::{borrow::Cow, sync::OnceLock};

fn rule_1(input: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"^((?:[aeiou]|xr|yt)+.*)").unwrap());
    match re.replace_all(input, "${1}ay") {
        Cow::Borrowed(_) => None,
        Cow::Owned(s) => Some(s),
    }
}

fn rule_2(input: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"^([^aeiou]+)(.*)").unwrap());
    match re.replace_all(input, "$2${1}ay") {
        Cow::Borrowed(_) => None,
        Cow::Owned(s) => Some(s),
    }
}

fn rule_3(input: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"^([^aeiou]*qu)(.*)").unwrap());
    match re.replace_all(input, "$2${1}ay") {
        Cow::Borrowed(_) => None,
        Cow::Owned(s) => Some(s),
    }
}

fn rule_4(input: &str) -> Option<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"^([^aeiou]+)(y.*)").unwrap());
    match re.replace_all(input, "$2${1}ay") {
        Cow::Borrowed(_) => None,
        Cow::Owned(s) => Some(s),
    }
}

pub fn translate(input: &str) -> String {
    let rules = [rule_1, rule_3, rule_4, rule_2];
    input
        .split(' ')
        .map(|s| {
            rules
                .iter()
                .find_map(|&rule| rule(s))
                .unwrap_or(String::from(""))
        })
        .collect::<Vec<_>>()
        .join(" ")
}
