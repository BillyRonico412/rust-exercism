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

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let mut counter = Counter::default();
    dice.iter().for_each(|&d| counter.increment(d));

    let compute_basic_category = |n| counter.get(n) * n;

    match category {
        Category::Ones => compute_basic_category(1),
        Category::Twos => compute_basic_category(2),
        Category::Threes => compute_basic_category(3),
        Category::Fours => compute_basic_category(4),
        Category::Fives => compute_basic_category(5),
        Category::Sixes => compute_basic_category(6),

        Category::FullHouse => {
            if counter.counts.contains(&3) && counter.counts.contains(&2) {
                dice.iter().sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => (1u8..=6u8)
            .find(|&d| counter.get(d) >= 4)
            .map_or(0, |d| d * 4),

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
