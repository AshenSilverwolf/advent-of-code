use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    IResult, Parser,
};
use std::collections::BTreeMap;

const ROCKS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

#[derive(Debug)]
enum Rock {
    Rock,
    Gap,
}

#[derive(Debug)]
struct RockFormation(Vec<Vec<Rock>>);

impl RockFormation {
    fn height(&self) -> u32 {
        let RockFormation(rocks) = self;
        rocks.len() as u32
    }
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

fn rocks(input: &str) -> IResult<&str, Vec<RockFormation>> {
    separated_list1(
        tag("\n\n"),
        separated_list1(
            line_ending,
            many1(alt((
                complete::char('#').map(|_| Rock::Rock),
                complete::char('.').map(|_| Rock::Gap),
            ))),
        )
        .map(|rocks| RockFormation(rocks)),
    )(input)
}

fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        complete::char('<').map(|_| Move::Left),
        complete::char('>').map(|_| Move::Right),
    )))(input)
}

pub fn process_part1(input: &str) -> String {
    let rock_limit = 2022;
    let mut rocks_stopped: u32 = 0;
    let (_, moves) = moves(input).unwrap();
    let (_, rocks) = rocks(ROCKS).unwrap();
    let mut rocks = rocks.iter().cycle();
    let mut moves = moves.iter().cycle();
    let field: BTreeMap<(u32, u32), Rock> = BTreeMap::new();
    while rocks_stopped != 2022 {
        let current_rock = rocks.next().unwrap();
        let max_rock_height = field
            .keys()
            .map(|(_, y)| y)
            .max()
            .unwrap_or(&0);
        let current_rock_position: (u32, u32) = (
            2,
            max_rock_height + 3 + current_rock.height(),
        );
        // while let Some() = field
        loop {
            let next_move = moves.next().unwrap();
            let current_position = match next_move {
                Move::Left => {
                    if current_rock_position.0 == 0 {
                        current_rock_position
                    } else {
                        (
                            current_rock_position.0 - 1,
                            current_rock_position.1,
                        )
                    }
                }
                Move::Right => {
                    if current_rock_position.0 == 6 {
                        current_rock_position
                    } else {
                        (
                            current_rock_position.0 + 1,
                            current_rock_position.1,
                        )
                    }
                }
            };
        }
    }

    field.keys().map(|(x, y)| y).max().unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    todo!("two")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "3068");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "3068");
    }
}
