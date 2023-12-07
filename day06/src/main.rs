use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

fn main() {
    let unpack = |(left, right)| (Hand::<0>::from_str(left), u32::from_str(right).unwrap());
    let mut data: Vec<_> = include_str!("../../data/input_day_7")
        .lines()
        .map(|x| x.split_once(' ').map(unpack).unwrap())
        .collect();
    data.sort_unstable_by_key(|(hand, __)| *hand);
    let ranks_sum = data
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (_, rank))| acc + (idx + 1) as u32 * rank);
    println!("res 1: {}", ranks_sum);

    let mut data: Vec<_> = data
        .into_iter()
        .map(|x| (x.0.into_second_part(), x.1))
        .collect();
    data.sort_unstable_by_key(|(hand, __)| *hand);

    let max_ranks_sum = data
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (_, rank))| acc + (idx + 1) as u32 * rank);
    println!("res 2: {}", max_ranks_sum)
    // res 1: 251216224
    // res 2: 250825971
}

#[derive(Debug, Copy, Clone)]
struct Card(char);

impl Card {
    fn card_value(&self) -> u8 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '2'..='9' => self.0.to_digit(10).unwrap() as u8,
            _ => unreachable!(),
        }
    }

    fn is_joker(&self) -> bool {
        self.0 == 'J'
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.card_value() == other.card_value()
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.card_value().cmp(&other.card_value())
    }
}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Copy, Clone)]
struct Hand<const PART: u8>([Card; 5]);

impl<const PART: u8> Hand<PART> {
    fn from_str(s: &str) -> Self {
        let arr = s.chars().map(Card).collect::<Vec<_>>().try_into().unwrap();
        Self(arr)
    }

    fn rank(&self) -> HandRank {
        let mut cards = HashMap::new();
        for card in self.0.iter() {
            *cards.entry(card).or_insert(0) += 1u8;
        }
        match cards.len() {
            5 => HandRank::HighCard,
            4 => HandRank::OnePair,
            3 => {
                if cards.values().any(|&x| x == 3) {
                    HandRank::ThreeOfAKind
                } else {
                    HandRank::TwoPair
                }
            }
            2 => {
                if cards.values().any(|&x| x == 4) {
                    HandRank::FourOfAKind
                } else {
                    HandRank::FullHouse
                }
            }
            1 => HandRank::FiveOfAKind,
            _ => unreachable!(),
        }
    }

    fn max_rank(&self) -> HandRank {
        let joker_count = self.0.iter().filter(|x| x.is_joker()).count() as u8;
        let rank = self.rank();
        if joker_count == 0 {
            return rank;
        }

        match (rank, joker_count) {
            (HandRank::FiveOfAKind, _) => HandRank::FiveOfAKind,
            (HandRank::FourOfAKind, _) => HandRank::FiveOfAKind,
            (HandRank::FullHouse, _) => HandRank::FiveOfAKind,
            (HandRank::ThreeOfAKind, _) => HandRank::FourOfAKind,
            (HandRank::TwoPair, 1) => HandRank::FullHouse,
            (HandRank::TwoPair, _) => HandRank::FourOfAKind,
            (HandRank::OnePair, _) => HandRank::ThreeOfAKind,
            (HandRank::HighCard, _) if joker_count >= 4 => HandRank::FiveOfAKind,
            (HandRank::HighCard, _) if joker_count == 3 => HandRank::FourOfAKind,
            (HandRank::HighCard, _) if joker_count == 2 => HandRank::ThreeOfAKind,
            (HandRank::HighCard, _) if joker_count == 1 => HandRank::OnePair,
            _ => rank.increment_by(joker_count).unwrap_or(rank),
        }
    }

    fn into_second_part(self) -> Hand<1> {
        let Hand(cards) = self;
        Hand::<1>(cards)
    }
}

impl<const PART: u8> std::fmt::Display for Hand<PART> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Hand(cards) = self;
        write!(f, "{}", cards.iter().map(|x| x.0).collect::<String>())
    }
}

#[rustfmt::skip]
impl<const PART: u8> Ord for Hand<PART> {
    fn cmp(&self, other: &Self) -> Ordering {
        let left_rank = if PART == 0 { self.rank() } else { self.max_rank() };
        let right_rank = if PART == 0 { other.rank() } else { other.max_rank() };

        if left_rank != right_rank {
            return left_rank.cmp(&right_rank);
        }
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(Ordering::Equal, |acc, (left, right)| {
                if acc != Ordering::Equal {
                    return acc;
                }
                if PART == 1 {
                    // if part 1 joker is lower than any card
                    return match (left.is_joker(), right.is_joker()) {
                        (true, true) => return Ordering::Equal,
                        (true, false) => return Ordering::Less,
                        (false, true) => return Ordering::Greater,
                        _ => left.cmp(right),
                    };
                }

                left.cmp(right)
            })
    }
}

impl<const PART: u8> PartialOrd for Hand<PART> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const PART: u8> PartialEq for Hand<PART> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<const PART: u8> Eq for Hand<PART> {}

#[derive(strum::FromRepr, Debug, PartialEq, Ord, PartialOrd, Eq, Clone, Copy)]
#[repr(u8)]
enum HandRank {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl HandRank {
    fn increment_by(self, count: u8) -> Option<Self> {
        HandRank::from_repr(self as u8 + count)
    }
}
