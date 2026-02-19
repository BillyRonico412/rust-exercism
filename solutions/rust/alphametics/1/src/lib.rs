use std::collections::{HashMap, HashSet};

struct Alphametics<'a> {
    left: Vec<&'a str>,
    right: &'a str,
    all_letters_map: HashMap<char, u8>,
    all_letters_vec: Vec<char>,
    first_letters: HashSet<char>,
    used_digits: [Option<char>; 10],
}

impl<'a> Alphametics<'a> {
    fn new(input: &'a str) -> Self {
        let mut split_by_equal = input.split("==");

        let left = split_by_equal
            .next()
            .unwrap()
            .split("+")
            .map(|l| l.trim())
            .collect::<Vec<&str>>();

        let right = split_by_equal.next().unwrap().trim();

        let mut all_letters_map = HashMap::new();
        let mut all_letters_vec = Vec::new();
        let mut first_letters = HashSet::new();
        input
            .chars()
            .filter(|c| c.is_ascii_uppercase())
            .for_each(|c| {
                all_letters_map.insert(c, 0u8);
                if !all_letters_vec.contains(&c) {
                    all_letters_vec.push(c);
                }
            });

        left.iter().for_each(|s| {
            if s.len() > 1 {
                first_letters.insert(s.chars().next().unwrap());
            }
        });
        if right.len() > 1 {
            first_letters.insert(right.chars().next().unwrap());
        }

        Alphametics {
            all_letters_map,
            all_letters_vec,
            first_letters,
            left,
            right,
            used_digits: [None; 10],
        }
    }

    fn word_to_number(&self, word: &str) -> u128 {
        word.chars()
            .enumerate()
            .map(|(i, c)| {
                *self.all_letters_map.get(&c).unwrap() as u128
                    * 10u128.pow((word.len() - i - 1) as u32)
            })
            .sum()
    }

    fn check_all(&mut self, depth: u8) -> bool {
        if depth == self.all_letters_vec.len() as u8 {
            return self.check_one();
        }
        for d in 0u8..=9u8 {
            if self.used_digits[d as usize].is_some() {
                continue;
            }
            let c = self.all_letters_vec[depth as usize];
            if self.first_letters.contains(&c) && d == 0 {
                continue;
            }
            self.used_digits[d as usize] = Some(c);
            if self.check_all(depth + 1) {
                return true;
            }
            self.used_digits[d as usize] = None;
        }
        false
    }

    fn check_one(&mut self) -> bool {
        // Generate all_letters_map
        self.used_digits
            .iter()
            .enumerate()
            .filter_map(|(digit_used, c)| c.map(|c| (digit_used, c)))
            .for_each(|(digit_used, c)| {
                self.all_letters_map.insert(c, digit_used as u8);
            });

        // Check sum
        self.left
            .iter()
            .map(|&word| self.word_to_number(word))
            .fold(0, |acc, word_number| acc + word_number)
            == self.word_to_number(self.right)
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut alphametics = Alphametics::new(input);
    if alphametics.check_all(0) {
        Some(alphametics.all_letters_map)
    } else {
        None
    }
}
