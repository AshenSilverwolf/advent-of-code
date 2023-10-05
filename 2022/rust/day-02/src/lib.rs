use std::{cmp::Ordering, str::FromStr};
use Ordering::*;

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Move::Rock, &Move::Rock) => Some(Equal),
            (&Move::Rock, &Move::Paper) => Some(Less),
            (&Move::Rock, &Move::Scissors) => Some(Greater),
            (&Move::Paper, &Move::Rock) => Some(Greater),
            (&Move::Paper, &Move::Paper) => Some(Equal),
            (&Move::Paper, &Move::Scissors) => Some(Less),
            (&Move::Scissors, &Move::Rock) => Some(Less),
            (&Move::Scissors, &Move::Paper) => Some(Greater),
            (&Move::Scissors, &Move::Scissors) => Some(Equal),
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(String::from("Invalid Move")),
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(' ')
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            match moves[0].partial_cmp(&moves[1]) {
                Some(Equal) => 3 + moves[1] as u32,
                Some(Less) => moves[1] as u32,
                Some(Greater) => 6 + moves[1] as u32,
                None => panic!("Moves should be comparable"),
            }
        })
        .sum();
    result.to_string()
}

pub fn process_part2(_input: &str) -> String {
    "two".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
