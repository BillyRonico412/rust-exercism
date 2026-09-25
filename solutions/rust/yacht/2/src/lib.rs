#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

#[derive(Default)]
struct Counter {
    counts: [u8; 6],
}

impl Counter {
    fn increment(&mut self, d: u8) {
        self.counts[(d - 1) as usize] += 1;
    }

    fn get(&self, d: u8) -> u8 {
        self.counts[(d - 1) as usize]
    }
}
impl Category {
    fn to_score(&self) -> u8 {
        match self {
            Category::Ones => 1,
            Category::Twos => 2,
            Category::Threes => 3,
            Category::Fours => 4,
            Category::Fives => 5,
            Category::Sixes => 6,
            _ => 0,
        }
    }
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let mut counter = Counter::default();
    dice.iter().for_each(|&d| counter.increment(d));

    match category {
        Category::Ones
        | Category::Twos
        | Category::Threes
        | Category::Fours
        | Category::Fives
        | Category::Sixes => counter.get(category.to_score()) * category.to_score(),

        Category::FullHouse => {
            if counter.counts.contains(&3) && counter.counts.contains(&2) {
                dice.iter().sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => (1u8..=6u8)
            .find(|&d| counter.get(d) >= 4)
            .map_or_default(|d| d * 4),

        Category::LittleStraight => {
            if counter.counts == [1, 1, 1, 1, 1, 0] {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            if counter.counts == [0, 1, 1, 1, 1, 1] {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            if counter.counts.iter().any(|&v| v == 5) {
                50
            } else {
                0
            }
        }
    }
}
