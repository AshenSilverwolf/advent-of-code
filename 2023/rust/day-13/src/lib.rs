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

fn double_newline(input: &str) -> IResult<&str, (char, char)> {
    tuple((newline, newline))(input)
}

fn pattern(input: &str) -> IResult<&str, Vec<Vec<Ground>>> {
    separated_list1(newline, many1(map(one_of(".#"), Ground::from)))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(double_newline, pattern)(input)
}

fn split_pattern_by_col(col: usize, pattern: &Pattern) -> (Pattern, Pattern) {
    assert!(col > 0);
    // separate the pattern into two parts by col
    // let mut first = pattern...
}

fn split_pattern_by_row(row: usize, pattern: &Pattern) -> (Pattern, Pattern) {
    assert!(row > 0);
    let mut first = pattern[..row].to_vec();
    let mut second = pattern[row..].to_vec();

    let f_len = first.len();
    let s_len = second.len();

    if f_len < s_len {
        let diff = s_len - f_len;
        let s_end = s_len - diff;
        second = second[..s_end].to_vec();
    } else if s_len < f_len {
        let diff = f_len - s_len;
        let f_start = diff;
        first = first[f_start..].to_vec();
    }

    second = second.into_iter().rev().collect();
    
    (first, second)
}

pub fn process_part1(input: &str) -> String {
    let (_, patterns) = parse_input(input).expect("valid input");

    for pattern in patterns.iter() {
        let (first, second) = split_pattern_by_col(5, pattern);
        dbg!(first == second);
    }
    
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
    #[ignore]
    fn part2_works() {
        let expected = String::from("");
        let result = process_part2(INPUT);
        assert_eq!(expected, result);
    }
}
