use nom::{
    IResult,
    character::complete::{self, newline, one_of},
    multi::{separated_list1, many1},
    combinator::map,
    bytes::complete::tag,
    sequence::separated_pair,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Condition {    
    fn from(value: char) -> Self {
        match value {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("invalid condition conversion"),
        }
    }
}

fn record(input: &str) -> IResult<&str, Vec<Condition>> {
    many1(map(one_of(".#?"), Condition::from))(input)
}

fn sequence(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), complete::u32)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Vec<Condition>, Vec<u32>)>> {
    separated_list1(
        newline,
        separated_pair(
            record,
            tag(" "),
            sequence,
        ),
    )(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, spring_rows) = parse_input(input).expect("valid input");

    dbg!(spring_rows);

    todo!()
}

pub fn process_part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1_works() {
        let expected = String::from("21");
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
