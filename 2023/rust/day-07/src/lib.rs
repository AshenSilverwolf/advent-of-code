// Modules?????

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::{cmp::Ordering, collections::BTreeMap};

// do not change
mod p1_types {
    use std::{cmp::Ordering, collections::BTreeMap};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Card {
        Deuce,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl TryFrom<char> for Card {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '2' => Ok(Card::Deuce),
                '3' => Ok(Card::Three),
                '4' => Ok(Card::Four),
                '5' => Ok(Card::Five),
                '6' => Ok(Card::Six),
                '7' => Ok(Card::Seven),
                '8' => Ok(Card::Eight),
                '9' => Ok(Card::Nine),
                'T' => Ok(Card::Ten),
                'J' => Ok(Card::Jack),
                'Q' => Ok(Card::Queen),
                'K' => Ok(Card::King),
                'A' => Ok(Card::Ace),
                _ => Err("Invalid card value"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandRank {
        HighCard,
        OnePair,
        TwoPair,
        Trips,
        FullHouse,
        Quads,
        Quints,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Hand(pub Vec<Card>);

    impl Hand {
        fn rank(&self) -> HandRank {
            let mut card_quantities: BTreeMap<Card, u8> = BTreeMap::new();
            for card in self.0.clone() {
                if let Some(x) = card_quantities.get_mut(&card) {
                    *x += 1;
                } else {
                    card_quantities.insert(card, 1);
                }
            }

            let mut card_counts = card_quantities.values().cloned().collect::<Vec<u8>>();
            card_counts.sort();
            let mut card_counts_hi_to_lo = card_counts.into_iter().rev();

            let Some(high) = card_counts_hi_to_lo.next() else {
                unreachable!()
            };
            match high {
                5 => HandRank::Quints,
                4 => HandRank::Quads,
                3 => {
                    if let Some(2) = card_counts_hi_to_lo.next() {
                        HandRank::FullHouse
                    } else {
                        HandRank::Trips
                    }
                }
                2 => {
                    if let Some(2) = card_counts_hi_to_lo.next() {
                        HandRank::TwoPair
                    } else {
                        HandRank::OnePair
                    }
                }
                1 => HandRank::HighCard,
                _ => unreachable!(),
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            let cmp_result = self.rank().cmp(&other.rank());
            match cmp_result {
                Ordering::Equal => self.0.cmp(&other.0),
                _ => cmp_result,
            }
        }
    }
}

// TODO?: modify to account for Jokers in part 2
mod p2_types {
    use std::{cmp::Ordering, collections::BTreeMap};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Card {
        Deuce,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl TryFrom<char> for Card {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '2' => Ok(Card::Deuce),
                '3' => Ok(Card::Three),
                '4' => Ok(Card::Four),
                '5' => Ok(Card::Five),
                '6' => Ok(Card::Six),
                '7' => Ok(Card::Seven),
                '8' => Ok(Card::Eight),
                '9' => Ok(Card::Nine),
                'T' => Ok(Card::Ten),
                'J' => Ok(Card::Jack),
                'Q' => Ok(Card::Queen),
                'K' => Ok(Card::King),
                'A' => Ok(Card::Ace),
                _ => Err("Invalid card value"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandRank {
        HighCard,
        OnePair,
        TwoPair,
        Trips,
        FullHouse,
        Quads,
        Quints,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Hand(pub Vec<Card>);

    impl Hand {
        fn rank(&self) -> HandRank {
            let mut card_quantities: BTreeMap<Card, u8> = BTreeMap::new();
            for card in self.0.clone() {
                if let Some(x) = card_quantities.get_mut(&card) {
                    *x += 1;
                } else {
                    card_quantities.insert(card, 1);
                }
            }

            let mut card_counts = card_quantities.values().cloned().collect::<Vec<u8>>();
            card_counts.sort();
            let mut card_counts_hi_to_lo = card_counts.into_iter().rev();

            let Some(high) = card_counts_hi_to_lo.next() else {
                unreachable!()
            };
            match high {
                5 => HandRank::Quints,
                4 => HandRank::Quads,
                3 => {
                    if let Some(2) = card_counts_hi_to_lo.next() {
                        HandRank::FullHouse
                    } else {
                        HandRank::Trips
                    }
                }
                2 => {
                    if let Some(2) = card_counts_hi_to_lo.next() {
                        HandRank::TwoPair
                    } else {
                        HandRank::OnePair
                    }
                }
                1 => HandRank::HighCard,
                _ => unreachable!(),
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            let cmp_result = self.rank().cmp(&other.rank());
            match cmp_result {
                Ordering::Equal => self.0.cmp(&other.0),
                _ => cmp_result,
            }
        }
    }
}

fn card(input: &str) -> IResult<&str, p1_types::Card> {
    map(one_of("23456789TJQKA"), |c: char| {
        p1_types::Card::try_from(c).expect("valid char")
    })(input)
}

fn hand(input: &str) -> IResult<&str, p1_types::Hand> {
    map(many1(card), |cards: Vec<p1_types::Card>| {
        p1_types::Hand(cards)
    })(input)
}

fn bid(input: &str) -> IResult<&str, u32> {
    complete::u32(input)
}

fn line(input: &str) -> IResult<&str, (p1_types::Hand, u32)> {
    separated_pair(hand, tag(" "), bid)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(p1_types::Hand, u32)>> {
    separated_list1(newline, line)(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, mut hands_bids) = parse_input(input).expect("valid input");
    hands_bids.sort_by_key(|(hand, _)| hand.clone());
    hands_bids
        .iter()
        .enumerate()
        .map(|(x, &(_, bid))| (x as u32 + 1) * bid)
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_works() {
        let expected = String::from("6440");
        let result = process_part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let expected = String::from("");
        let result = process_part2(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn cards_order() {
        assert!(p1_types::Card::Deuce < p1_types::Card::Three);
        assert!(p1_types::Card::Ace > p1_types::Card::Jack);
        assert!(p1_types::Card::Ten == p1_types::Card::Ten);
    }
}
