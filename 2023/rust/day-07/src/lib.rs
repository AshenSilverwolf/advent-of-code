use std::cmp::Ordering;
use nom::{
    IResult,
    character::complete::{self, newline, one_of},
    bytes::complete::tag,
    multi::{many1, separated_list1},
    combinator::map,
    sequence::separated_pair,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
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
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    Trips,
    FullHouse,
    Quads,
    Quints,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand(Vec<Card>);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp_result = self.rank().cmp(&other.rank());
        match cmp_result {
            Ordering::Equal => {
                // ranks are same, evaluate card by card
                return self.0.cmp(&other.0);
            }
            _ => {
                // ranks are different, return comparison
                return cmp_result;
            }
        }
    }
}

impl Hand {
    fn rank(&self) -> HandRank {
        todo!()
    }
}

fn card(input: &str) -> IResult<&str, Card> {
    map(
        one_of("23456789TJQKA"),
        |c: char| Card::try_from(c).expect("valid char")
    )(input)
}

fn hand(input: &str) -> IResult<&str, Hand> {
    map(
        many1(card),
        |cards: Vec<Card>| Hand(cards)
    )(input)
}

fn bid(input: &str) -> IResult<&str, u32> {
    complete::u32(input)
}

fn line(input: &str) -> IResult<&str, (Hand, u32)> {
    separated_pair(hand, tag(" "), bid)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Hand, u32)>> {
    separated_list1(newline, line)(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, mut before) = parse_input(input).expect("valid input");
    let after = before.sort_by_key(|(hand, _)| hand);
    dbg!(before, after);

    todo!()
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
        assert!(Card::Deuce < Card::Three);
        assert!(Card::Ace > Card::Jack);
        assert!(Card::Ten == Card::Ten);
    }
}
