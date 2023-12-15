use nom::{
    IResult,
    character::complete::{self, newline, space1, digit1},
    multi::separated_list1,
    bytes::complete::tag,
    sequence::{preceded, delimited},
};
use std::iter::zip;

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    record: u64,
}

fn times_or_dists(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

fn parse_one(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, times) = delimited(space1, times_or_dists, newline)(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, dists) = preceded(space1, times_or_dists)(input)?;
    assert_eq!(times.len(), dists.len());
    let iter = zip(times, dists);
    let output = iter.map(|(t, d)| Race { time: t, record: d }).collect();

    Ok((input, output))
}

fn parse_two(input: &str) -> IResult<&str, Race> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, time_nums) = separated_list1(space1, digit1)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, dist_nums) = separated_list1(space1, digit1)(input)?;
    assert_eq!(time_nums.len(), dist_nums.len());
    let time = time_nums.join("").parse::<u64>().expect("valid time");
    let record = dist_nums.join("").parse::<u64>().expect("valid record");

    Ok((input, Race { time, record }))
}

pub fn process_part1(input: &str) -> String {
    let (_, races) = parse_one(input).expect("valid input");

    races
        .iter()
        .map(|race| {
            let mut hold_times = 0;
            for i in 1..=race.time {
                let max_dist = i * (race.time - i);
                if max_dist > race.record {
                    hold_times += 1;
                }
            }

            hold_times
        })
        .product::<i32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, race) = parse_two(input).expect("valid input");
    let mut lo = 0;
    let mut hi = 0;

    // find lo
    for i in 1..=race.time {
        let max_dist = i * (race.time - i);
        if max_dist > race.record {
            lo = i;
            break;
        }
    }
    // find hi
    for i in (1..=race.time).rev() {
        let max_dist = i * (race.time - i);
        if max_dist > race.record {
            hi = i;
            break;
        }
    }

    let hold_times = hi - lo + 1;
    
    hold_times.to_string()
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
        let expected = String::from("71503");
        let result = process_part2(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn parse2_works() {
        let expected = Race { time: 71530, record: 940200 };
        let result = parse_two(INPUT).unwrap().1;
        assert_eq!(expected, result);
    }
}
