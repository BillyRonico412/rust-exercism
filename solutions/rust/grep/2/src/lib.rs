use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Error;

#[derive(Debug, PartialEq, Eq)]
enum Flag {
    N,
    L,
    I,
    V,
    X,
}

impl TryFrom<&str> for Flag {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "-n" => Ok(Flag::N),
            "-l" => Ok(Flag::L),
            "-i" => Ok(Flag::I),
            "-v" => Ok(Flag::V),
            "-x" => Ok(Flag::X),
            _ => Err("No matching flags".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Flags(Vec<Flag>);

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self(
            flags
                .iter()
                .filter_map(|&flag| Flag::try_from(flag).ok())
                .collect(),
        )
    }

    fn contains(&self, flag: &Flag) -> bool {
        self.0.contains(&flag)
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut res = vec![];

    for file_path in files {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut is_match_file = false;
        for (i, line) in reader.lines().enumerate() {
            let mut text = line?;
            let (left, right) = if flags.contains(&Flag::I) {
                (text.to_lowercase(), pattern.to_lowercase())
            } else {
                (text.to_string(), pattern.to_string())
            };
            let mut is_match = if flags.contains(&Flag::X) {
                left == right
            } else {
                left.contains(&right)
            };
            if flags.contains(&Flag::V) {
                is_match = !is_match
            }
            if !is_match {
                continue;
            }
            if flags.contains(&Flag::L) {
                is_match_file = true;
                break;
            }
            if flags.contains(&Flag::N) {
                text = format!("{}:{}", i + 1, text)
            }
            if files.len() > 1 {
                text = format!("{}:{}", file_path, text)
            }
            res.push(text);
        }
        if is_match_file {
            res.push(file_path.to_string());
        }
    }
    Ok(res)
}
