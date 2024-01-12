use nom::{
    IResult,
    multi::{separated_list1, many1},
    character::complete::{newline, one_of},
    combinator::map,
    sequence::tuple,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Ground {
    Ash,
    Rock,
}

type Pattern = Vec<Vec<Ground>>;

impl From<char> for Ground {
    fn from(c: char) -> Self {
        match c {
            '.' => Ground::Ash,
            '#' => Ground::Rock,
            _ => unreachable!(),
        }
    }
}

fn pattern(input: &str) -> IResult<&str, Vec<Vec<Ground>>> {
    separated_list1(newline, many1(map(one_of(".#"), Ground::from)))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(tuple((newline, newline)), pattern)(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, patterns) = parse_input(input).expect("valid input");

    dbg!(patterns);
    
    todo!()
}

pub fn process_part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_works() {
        let expected = String::from("");
        let result = process_part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn part2_works() {
        let expected = String::from("");
        let result = process_part2(INPUT);
        assert_eq!(expected, result);
    }
}
