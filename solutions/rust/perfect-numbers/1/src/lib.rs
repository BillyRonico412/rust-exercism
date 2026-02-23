#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn get_aliquot_sum(num: u64) -> u64 {
    (1..num)
        .filter_map(|x| if num % x != 0 { None } else { Some(x) })
        .sum()
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    match (num, get_aliquot_sum(num)) {
        (num, aliquot) if num > aliquot => Some(Classification::Deficient),
        (num, aliquot) if num < aliquot => Some(Classification::Abundant),
        _ => Some(Classification::Perfect),
    }
}
