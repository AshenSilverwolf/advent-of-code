use nom::{bytes::complete::tag, character::complete, IResult};
use std::ops::RangeInclusive;

type RangePair = (RangeInclusive<u32>, RangeInclusive<u32>);

fn range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = complete::u32(input)?;
    Ok((input, start..=end))
}

fn range_pair(input: &str) -> IResult<&str, RangePair> {
    let (input, first) = range(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, second) = range(input)?;
    Ok((input, (first, second)))
}

fn section_assignments(input: &str) -> IResult<&str, Vec<RangePair>> {
    let (input, ranges) = nom::multi::separated_list1(complete::newline, range_pair)(input)?;
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
    let (_, assignments) = section_assignments(input).unwrap();
    let result = assignments
        .iter()
        .filter(|(range_a, range_b)| {
            let a_contains_b = range_a.clone().any(|num| range_b.contains(&num));
            let b_contains_a = range_b.clone().any(|num| range_a.contains(&num));
            a_contains_b || b_contains_a
        })
        .count();
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
