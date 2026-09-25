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

struct DiceCount {
    value: u8,
    count: u8,
}

impl DiceCount {
    fn new(value: u8) -> Self {
        Self { value, count: 1 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }
}

pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones
        | Category::Twos
        | Category::Threes
        | Category::Fours
        | Category::Fives
        | Category::Sixes => dice.iter().fold(0, |acc, &d| {
            if d == category.to_score() {
                acc + category.to_score()
            } else {
                acc
            }
        }),
        Category::FullHouse => {
            let mut first = None;
            let mut second = None;

            for &d in dice.iter() {
                let Some(first) = first.as_mut() else {
                    first = Some(DiceCount::new(d));
                    continue;
                };

                if first.value == d {
                    first.increment();
                    if first.count > 3 {
                        return 0;
                    }
                    continue;
                }

                let Some(second) = second.as_mut() else {
                    second = Some(DiceCount::new(d));
                    continue;
                };

                if second.value == d {
                    second.increment();
                    if second.count > 3 {
                        return 0;
                    }
                    continue;
                }
                return 0;
            }

            dice.iter().sum()
        }
        Category::FourOfAKind => {
            let mut first = None;
            let mut second = None;
            for &d in dice.iter() {
                let Some(first) = first.as_mut() else {
                    first = Some(DiceCount::new(d));
                    continue;
                };

                if first.value == d {
                    first.increment();
                    continue;
                }

                let Some(second) = second.as_mut() else {
                    second = Some(DiceCount::new(d));
                    continue;
                };

                if second.value == d {
                    second.increment();
                    continue;
                }

                return 0;
            }

            if let Some(first) = first
                && first.count >= 4
            {
                return first.value * 4;
            }

            if let Some(second) = second
                && second.count >= 4
            {
                return second.value * 4;
            }

            return 0;
        }

        Category::LittleStraight => {
            let mut state = [false, false, false, false, false];
            for d in dice {
                if d == 6 {
                    return 0;
                }
                let Some(s) = state.get_mut((d - 1) as usize) else {
                    return 0;
                };
                if !*s {
                    *s = true
                } else {
                    return 0;
                }
            }
            30
        }
        Category::BigStraight => {
            let mut state = [false, false, false, false, false];
            for d in dice {
                if d == 1 {
                    return 0;
                }
                let Some(s) = state.get_mut((d - 2) as usize) else {
                    return 0;
                };
                if !*s {
                    *s = true
                } else {
                    return 0;
                }
            }
            30
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            if (1..=6).any(|d| dice == [d, d, d, d, d]) {
                50
            } else {
                0
            }
        }
    }
}
