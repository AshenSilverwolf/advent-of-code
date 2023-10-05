use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (Move::Scissors, Move::Rock) => Some(Ordering::Less),
            (Move::Rock, Move::Scissors) => Some(Ordering::Greater),
            _ => Some((*self as u8).cmp(&(*other as u8))),
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(String::from("Invalid Move")),
        }
    }
}

enum Result {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl FromStr for Result {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "X" => Ok(Result::Loss),
            "Y" => Ok(Result::Draw),
            "Z" => Ok(Result::Win),
            _ => Err(String::from("Invalid Result")),
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
                Some(Ordering::Equal) => 3 + moves[1] as u32,
                Some(Ordering::Less) => 6 + moves[1] as u32,
                Some(Ordering::Greater) => moves[1] as u32,
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
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
