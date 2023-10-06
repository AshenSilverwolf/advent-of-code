use nom::{bytes::complete::tag, character::complete, IResult};
use std::collections::HashSet;
use std::ops::RangeInclusive;

type RangePair = (RangeInclusive<u32>, RangeInclusive<u32>);

fn sections(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = complete::u32(input)?;
    Ok((input, start..=end))
}

fn line(input: &str) -> IResult<&str, RangePair> {
    let (input, start) = sections(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, end) = sections(input)?;
    Ok((input, (start, end)))
}

fn section_assignments(input: &str) -> IResult<&str, Vec<RangePair>> {
    let (input, ranges) = nom::multi::separated_list1(complete::newline, line)(input)?;
    Ok((input, ranges))
}

pub fn process_part1(input: &str) -> String {
    let (_, assignments) = section_assignments(input).unwrap();
    let result = assignments
        .iter()
        .filter(|(range_a, range_b)| {
            let a_contains_b = range_a.clone().all(|num| range_b.contains(&num));
            let b_contains_a = range_b.clone().all(|num| range_a.contains(&num));
            a_contains_b || b_contains_a
        })
        .count();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|s| {
            let mut zones = s.split(',');
            let first: &str = zones.next().unwrap();
            let second: &str = zones.next().unwrap();
            let mut zone_one = first.split('-');
            let mut zone_two = second.split('-');
            let one_one: u32 = zone_one.next().unwrap().parse::<u32>().unwrap();
            let one_two: u32 = zone_one.next().unwrap().parse::<u32>().unwrap();
            let two_one: u32 = zone_two.next().unwrap().parse::<u32>().unwrap();
            let two_two: u32 = zone_two.next().unwrap().parse::<u32>().unwrap();
            let set_one: HashSet<u32> = (one_one..=one_two).collect::<HashSet<u32>>();
            let set_two: HashSet<u32> = (two_one..=two_two).collect::<HashSet<u32>>();
            if set_one.intersection(&set_two).count() > 0 {
                1
            } else {
                0
            }
        })
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
