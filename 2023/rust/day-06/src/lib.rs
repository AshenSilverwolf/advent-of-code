use nom::{
    IResult,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    bytes::complete::tag,
    sequence::{preceded, delimited},
};
use std::iter::zip;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn times_or_dists(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, complete::u32)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, times) = delimited(space1, times_or_dists, newline)(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, dists) = preceded(space1, times_or_dists)(input)?;
    assert_eq!(times.len(), dists.len());
    let iter = zip(times, dists);
    let output = iter.map(|(t, d)| Race { time: t, distance: d }).collect();

    Ok((input, output))
}

pub fn process_part1(input: &str) -> String {
    let (input, races) = parse_input(input).expect("valid input");
    dbg!(races);

    todo!()
}

pub fn process_part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_works() {
        let expected = String::from("288");
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
