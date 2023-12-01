use std::collections::BTreeMap;

use nom::{
    bytes::complete::{is_not, take_until, take_until1, take_while1},
    character::{
        complete::{newline, none_of},
        is_alphanumeric,
    },
    multi::{many1, separated_list1},
    IResult,
};

pub fn process_part1(input: &str) -> IResult<&str, String> {
    let (input, calibration_lines) = separated_list1(newline, many1(none_of("\n")))(input)?;

    let output = calibration_lines
        .iter()
        .map(|v| {
            let nums: Vec<u32> = v
                .iter()
                .filter_map(|c| c.to_string().parse::<u32>().ok())
                .collect();
            let mut nums_iter = nums.iter();

            let first = nums_iter.next();
            let last = nums_iter.last();

            match (first, last) {
                (Some(x), Some(y)) => 10 * x + y,
                (Some(x), None) => 10 * x + x,
                _ => panic!("ruh roh"),
            }
        })
        .sum::<u32>()
        .to_string();

    Ok((input, output))
}

// u32 from &str
// - first letter -> possible word lengths
// - check windows for valid text
// - if yes, parse the value
// - if no, move on
fn u32_from_chars(chars: &str) -> Vec<u32> {
    dbg!("entered");
    let mut letter_lengths: BTreeMap<&str, u32> = BTreeMap::new();
    letter_lengths.insert("one", 1);
    letter_lengths.insert("two", 2);
    letter_lengths.insert("three", 3);
    letter_lengths.insert("four", 4);
    letter_lengths.insert("five", 5);
    letter_lengths.insert("six", 6);
    letter_lengths.insert("seven", 7);
    letter_lengths.insert("eight", 8);
    letter_lengths.insert("nine", 9);

    let mut output: Vec<u32> = vec![];

    for i in 0..chars.len() {
        let curr = &chars[i..];
        if let Ok(x) = curr.get(0..1).unwrap().parse::<u32>() {
            output.push(x);
            continue;
        }
        for s in letter_lengths.keys() {
            if curr.starts_with(s) {
                output.push(letter_lengths.get(s).unwrap().to_owned());
            }
        }
        dbg!(&output);
    }

    output
}

pub fn process_part2(input: &str) -> IResult<&str, String> {
    let (input, calibration_lines) = separated_list1(newline, is_not("\n"))(input)?;
    let output = calibration_lines
        .iter()
        .map(|s| {
            let nums = u32_from_chars(s);
            let mut nums_iter = nums.iter();

            let first = nums_iter.next();
            let last = nums_iter.last();

            match (first, last) {
                (Some(x), Some(y)) => x * 10 + y,
                (Some(x), None) => x * 10 + x,
                _ => panic!("ruh roh"),
            }
        })
        .sum::<u32>()
        .to_string();

    Ok((input, output))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part1_works() {
        let expected = String::from("142");
        let (_, result) = process_part1(INPUT_1).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn part2_works() {
        let expected = String::from("281");
        let (_, result) = process_part2(INPUT_2).unwrap();
        assert_eq!(expected, result);
    }
}
