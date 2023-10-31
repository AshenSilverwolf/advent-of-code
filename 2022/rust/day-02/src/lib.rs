use std::str::FromStr;

#[derive(Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(String::from("Invalid Move")),
        }
    }
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
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
            match (moves[0], moves[1]) {
                (Move::Rock, Move::Rock) => moves[1] as u32 + 3,
                (Move::Rock, Move::Paper) => moves[1] as u32 + 6,
                (Move::Rock, Move::Scissors) => moves[1] as u32,
                (Move::Paper, Move::Rock) => moves[1] as u32,
                (Move::Paper, Move::Paper) => moves[1] as u32 + 3,
                (Move::Paper, Move::Scissors) => moves[1] as u32 + 6,
                (Move::Scissors, Move::Rock) => moves[1] as u32 + 6,
                (Move::Scissors, Move::Paper) => moves[1] as u32,
                (Move::Scissors, Move::Scissors) => moves[1] as u32 + 3,
            }
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let items: Vec<&str> = line.split(' ').collect();
            let elf: Move = items[0].parse().unwrap();
            let outcome: Outcome = items[1].parse().unwrap();
            match (elf, outcome) {
                (Move::Rock, Outcome::Win) => Move::Paper as u32 + 6,
                (Move::Rock, Outcome::Draw) => Move::Rock as u32 + 3,
                (Move::Rock, Outcome::Loss) => Move::Scissors as u32,
                (Move::Paper, Outcome::Win) => Move::Scissors as u32 + 6,
                (Move::Paper, Outcome::Draw) => Move::Paper as u32 + 3,
                (Move::Paper, Outcome::Loss) => Move::Rock as u32,
                (Move::Scissors, Outcome::Win) => Move::Rock as u32 + 6,
                (Move::Scissors, Outcome::Draw) => Move::Scissors as u32 + 3,
                (Move::Scissors, Outcome::Loss) => Move::Paper as u32,
            }
        })
        .sum();
    result.to_string()
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
