use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl From<&str> for CubeColor {
    fn from(value: &str) -> Self {
        match value {
            "red" => CubeColor::Red,
            "green" => CubeColor::Green,
            _ => CubeColor::Blue,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn zero() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn possible(&self, reference: &Self) -> bool {
        self.red <= reference.red && self.green <= reference.green && self.blue <= reference.blue
    }
}

impl From<Vec<(u32, CubeColor)>> for Round {
    fn from(value: Vec<(u32, CubeColor)>) -> Self {
        let mut output = Self::zero();
        for (num, color) in value {
            match color {
                CubeColor::Red => output.red = num,
                CubeColor::Green => output.green = num,
                CubeColor::Blue => output.blue = num,
            };
        }

        output
    }
}

fn num_cubes(input: &str) -> IResult<&str, (u32, CubeColor)> {
    pair(
        complete::u32,
        preceded(
            tag(" "),
            map(
                alt((tag("red"), tag("green"), tag("blue"))),
                CubeColor::from,
            ),
        ),
    )(input)
}

fn round(input: &str) -> IResult<&str, Round> {
    map(separated_list1(tag(", "), num_cubes), Round::from)(input)
}

fn prefix(input: &str) -> IResult<&str, u16> {
    delimited(tag("Game "), complete::u16, tag(": "))(input)
}

fn game(input: &str) -> IResult<&str, (u16, Vec<Round>)> {
    tuple((prefix, separated_list1(tag("; "), round)))(input)
}

pub fn process_part1(input: &str) -> String {
    let reference = Round {
        red: 12,
        green: 13,
        blue: 14,
    };
    let (_, games) = separated_list1(newline, game)(input).unwrap();
    games
        .iter()
        .filter_map(|(id, game)| {
            if game.iter().all(|round| round.possible(&reference)) {
                Some(id)
            } else {
                None
            }
        })
        .sum::<u16>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, games) = separated_list1(newline, game)(input).unwrap();
    games
        .iter()
        .map(|(_, game)| {
            let mut max = Round::zero();
            for round in game {
                if round.red > max.red {
                    max.red = round.red;
                }
                if round.green > max.green {
                    max.green = round.green;
                }
                if round.blue > max.blue {
                    max.blue = round.blue;
                }
            }
            max.red * max.green * max.blue
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_works() {
        let expected = String::from("8");
        let result = process_part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn part2_works() {
        let expected = String::from("2286");
        let result = process_part2(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn color_from_str() {
        let expected = CubeColor::Red;
        let result = CubeColor::from("red");
        assert_eq!(expected, result);
    }

    #[test]
    fn round_from_vec() {
        let expected = Round {
            red: 1,
            green: 2,
            blue: 4,
        };
        let result = Round::from(vec![
            (2, CubeColor::Green),
            (1, CubeColor::Red),
            (4, CubeColor::Blue),
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn game_is_possible() {
        let test = Round {
            red: 1,
            green: 1,
            blue: 1,
        };
        let reference = Round {
            red: 10,
            green: 10,
            blue: 10,
        };
        let expected = true;
        let result = test.possible(&reference);
        assert_eq!(expected, result);
    }

    #[test]
    fn game_is_not_possible() {
        let test = Round {
            red: 10,
            green: 10,
            blue: 10,
        };
        let reference = Round {
            red: 1,
            green: 1,
            blue: 1,
        };
        let expected = false;
        let result = test.possible(&reference);
        assert_eq!(expected, result);
    }
}
