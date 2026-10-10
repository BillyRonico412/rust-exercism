use std::collections::BTreeMap;

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardSuit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl TryFrom<char> for CardSuit {
    type Error = ();
    fn try_from(c: char) -> Result<CardSuit, Self::Error> {
        match c {
            'C' => Ok(Self::Club),
            'D' => Ok(Self::Diamond),
            'H' => Ok(Self::Heart),
            'S' => Ok(Self::Spade),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum CardValue {
    Number(u32),
    Jack,
    Queen,
    King,
    As,
}

impl From<&CardValue> for u32 {
    fn from(value: &CardValue) -> Self {
        match value {
            CardValue::Number(n) => *n,
            CardValue::Jack => 11,
            CardValue::Queen => 12,
            CardValue::King => 13,
            CardValue::As => 14,
        }
    }
}

struct Card {
    value: CardValue,
    suit: CardSuit,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl TryFrom<&str> for Card {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut chars = s.chars();
        match (chars.next(), chars.next(), chars.next()) {
            (Some('1'), Some('0'), Some(suit)) => Ok(Card {
                value: CardValue::Number(10),
                suit: suit.try_into().unwrap(),
            }),
            (Some(n @ '2'..='9'), Some(suit), None) => Ok(Card {
                value: CardValue::Number(n.to_digit(10).unwrap()),
                suit: suit.try_into().unwrap(),
            }),
            (Some('J'), Some(suit), None) => Ok(Card {
                value: CardValue::Jack,
                suit: suit.try_into().unwrap(),
            }),
            (Some('Q'), Some(suit), None) => Ok(Card {
                value: CardValue::Queen,
                suit: suit.try_into().unwrap(),
            }),
            (Some('K'), Some(suit), None) => Ok(Card {
                value: CardValue::King,
                suit: suit.try_into().unwrap(),
            }),
            (Some('A'), Some(suit), None) => Ok(Card {
                value: CardValue::As,
                suit: suit.try_into().unwrap(),
            }),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    QuinteFlush,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    rank: HandRank,
    values: Vec<u32>,
}

#[derive(PartialEq)]
struct Cards(Vec<Card>);

impl Cards {
    fn cards_map(&self) -> BTreeMap<CardValue, u32> {
        self.0.iter().fold(BTreeMap::new(), |mut acc, c| {
            *acc.entry(c.value.clone()).or_insert(0) += 1;
            acc
        })
    }

    fn quinte_flush(&self) -> Option<Vec<u32>> {
        let is_same_suit = self.0.iter().map(|c| &c.suit).all_equal();
        if !is_same_suit {
            return None;
        }
        self.straight()
    }

    fn four_of_a_kind(&self) -> Option<Vec<u32>> {
        let cards_map = self.cards_map();
        let four = cards_map
            .iter()
            .find_map(|(key, value)| (*value == 4).then_some(key));
        let Some(four) = four else {
            return None;
        };
        let kicker = self.0.iter().find(|c| c.value != *four);
        let Some(kicker) = kicker else {
            return None;
        };
        Some(
            [four, &kicker.value]
                .map(|v| u32::from(v))
                .into_iter()
                .collect_vec(),
        )
    }

    fn full_house(&self) -> Option<Vec<u32>> {
        let three_of_a_kind = self.three_of_a_kind();
        let Some(three_of_a_kind) = three_of_a_kind else {
            return None;
        };
        let one_pair = self.one_pair();
        let Some(one_pair) = one_pair else {
            return None;
        };
        Some(vec![three_of_a_kind[0], one_pair[0]])
    }

    fn flush(&self) -> Option<Vec<u32>> {
        if !self.0.iter().map(|c| c.suit).all_equal() {
            return None;
        }
        Some(self.0.iter().rev().map(|c| u32::from(&c.value)).collect())
    }

    fn straight(&self) -> Option<Vec<u32>> {
        let is_special = self.0.iter().map(|c| c.value).eq([
            CardValue::Number(2),
            CardValue::Number(3),
            CardValue::Number(4),
            CardValue::Number(5),
            CardValue::As,
        ]);
        if is_special {
            return Some(
                [
                    CardValue::As,
                    CardValue::Number(2),
                    CardValue::Number(3),
                    CardValue::Number(4),
                    CardValue::Number(5),
                ]
                .iter()
                .rev()
                .map(|v| u32::from(v))
                .collect_vec(),
            );
        }
        let is_consecutive = self
            .0
            .iter()
            .map(|c| &c.value)
            .tuple_windows()
            .all(|(a, b)| u32::from(a) + 1 == u32::from(b));
        if !is_consecutive {
            return None;
        }
        Some(self.0.iter().rev().map(|c| u32::from(&c.value)).collect())
    }

    fn three_of_a_kind(&self) -> Option<Vec<u32>> {
        let cards_map = self.cards_map();
        let three = cards_map
            .iter()
            .find_map(|(key, value)| (*value == 3).then_some(key));
        let Some(three) = three else {
            return None;
        };
        let mut kickers = self
            .0
            .iter()
            .filter_map(|c| (c.value != *three).then_some(c.value))
            .rev();
        let (Some(kicker1), Some(kicker2)) = (kickers.next(), kickers.next()) else {
            return None;
        };
        Some(
            [three, &kicker1, &kicker2]
                .map(|v| u32::from(v))
                .into_iter()
                .collect_vec(),
        )
    }

    fn one_pair(&self) -> Option<Vec<u32>> {
        let cards_map = self.cards_map();
        let pair = cards_map
            .iter()
            .find_map(|(key, value)| (*value == 2).then_some(key));

        let Some(pair) = pair else {
            return None;
        };

        let mut kickers = self
            .0
            .iter()
            .filter_map(|c| (c.value != *pair).then_some(c.value))
            .rev();

        let (Some(kicker1), Some(kicker2), Some(kicker3)) =
            (kickers.next(), kickers.next(), kickers.next())
        else {
            return None;
        };

        Some(
            [pair, &kicker1, &kicker2, &kicker3]
                .map(|v| u32::from(v))
                .into_iter()
                .collect_vec(),
        )
    }

    fn two_pair(&self) -> Option<Vec<u32>> {
        let cards_map = self.cards_map();
        let mut pairs = cards_map
            .iter()
            .filter_map(|(key, value)| (*value == 2).then_some(key));

        let (Some(pair1), Some(pair2)) = (pairs.next(), pairs.next()) else {
            return None;
        };

        let kicker = self
            .0
            .iter()
            .find_map(|c| (c.value != *pair1 && c.value != *pair2).then_some(c.value));

        let Some(kicker) = kicker else {
            return None;
        };

        Some(
            [pair2, pair1, &kicker]
                .map(|v| u32::from(v))
                .into_iter()
                .collect_vec(),
        )
    }

    fn high_card(&self) -> Vec<u32> {
        self.0.iter().rev().map(|c| u32::from(&c.value)).collect()
    }
}

impl TryFrom<&str> for Cards {
    type Error = ();

    fn try_from(hands: &str) -> Result<Self, Self::Error> {
        let cards = hands
            .split(" ")
            .map(|c| Card::try_from(c).unwrap())
            .sorted()
            .collect_vec();
        Ok(Self(cards))
    }
}

impl From<Cards> for Hand {
    fn from(cards: Cards) -> Self {
        let hand_rankings: [(HandRank, fn(&Cards) -> Option<Vec<u32>>); 8] = [
            (HandRank::QuinteFlush, Cards::quinte_flush),
            (HandRank::FourOfAKind, Cards::four_of_a_kind),
            (HandRank::FullHouse, Cards::full_house),
            (HandRank::Flush, Cards::flush),
            (HandRank::Straight, Cards::straight),
            (HandRank::ThreeOfAKind, Cards::three_of_a_kind),
            (HandRank::TwoPair, Cards::two_pair),
            (HandRank::OnePair, Cards::one_pair),
        ];

        let (rank, values) = hand_rankings
            .iter()
            .find_map(|(r, f)| f(&cards).map(|values| (*r, values)))
            .unwrap_or((HandRank::HighCard, cards.high_card()));

        Self { rank, values }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let best = hands
        .iter()
        .sorted_by(|a, b| {
            let card_a = Cards::try_from(**a).unwrap();
            let hand_a = Hand::from(card_a);
            let card_b = Cards::try_from(**b).unwrap();
            let hand_b = Hand::from(card_b);
            hand_a.cmp(&hand_b)
        })
        .rev()
        .next()
        .map(|&s| Cards::try_from(s).unwrap());
    let Some(best) = best else {
        return vec![];
    };
    hands
        .iter()
        .filter(|c| Cards::try_from(**c).unwrap() == best)
        .copied()
        .collect()
}
