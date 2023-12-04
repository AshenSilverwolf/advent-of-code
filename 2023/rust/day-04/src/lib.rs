use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone)]
struct Card {
    winning: Vec<u32>,
    yours: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let num_wins = self
            .yours
            .iter()
            .filter(|n| self.winning.contains(n))
            .count();
        if num_wins == 0 {
            0
        } else {
            u32::pow(2, num_wins as u32 - 1)
        }
    }

    fn wins(&self) -> u32 {
        self.yours
            .iter()
            .filter(|n| self.winning.contains(n))
            .count() as u32
    }
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(many1(complete::char(' ')), complete::u32)(input)
}

fn card(input: &str) -> IResult<&str, (u32, Card)> {
    let (input, id) = delimited(
        tuple((tag("Card"), many1(complete::char(' ')))),
        complete::u32,
        tag(": "),
    )(input)?;
    let (input, (winning, yours)) = separated_pair(
        preceded(many0(complete::char(' ')), numbers),
        tag(" | "),
        preceded(many0(complete::char(' ')), numbers),
    )(input)?;

    Ok((input, (id, Card { winning, yours })))
}

pub fn process_part1(input: &str) -> String {
    let (_, cards) = separated_list1(newline, card)(input).expect("valid input");
    cards
        .iter()
        .map(|(_, card)| card.score())
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, cards) = separated_list1(newline, card)(input).expect("valid input");
    let mut q: VecDeque<(u32, Card)> = VecDeque::from(cards.clone());
    let mut map: BTreeMap<u32, Card> = BTreeMap::new();
    let mut num_cards: u32 = 0;
    for (id, card) in cards {
        map.insert(id, card);
    }

    while let Some((id, card)) = q.pop_front() {
        num_cards += 1;
        let num_wins = card.wins();
        for i in (id + 1)..=(id + num_wins) {
            q.push_back((i, map.get(&i).expect("valid card").clone()));
        }
    }

    num_cards.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_works() {
        let expected = String::from("13");
        let result = process_part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn part2_works() {
        let expected = String::from("30");
        let result = process_part2(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn numbers_works() {
        let expected: Vec<u32> = vec![42, 1, 18, 2, 24, 3, 25];
        let input = "42  1 18  2 24  3 25";
        let (_, result) = numbers(input).expect("valid Vec");
        assert_eq!(expected, result);
    }

    #[test]
    fn impl_card_wins() {
        let expected: u32 = 1;
        let card = Card {
            winning: vec![1, 2, 3],
            yours: vec![1],
        };
        let result: u32 = card.wins();
        assert_eq!(expected, result);
    }
}
