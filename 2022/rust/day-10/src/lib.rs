use nom::*;
use nom::multi::separated_list1;
use nom::combinator::{opt, map_res};
use nom::branch::alt;
use nom::character::complete::{newline, digit1};
use nom::bytes::complete::tag;

#[derive(Debug)]
enum Operation {
    Addx(i32),
    Noop,
}

fn addx(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("addx ")(input)?;
    let (input, minus) = opt(tag("-"))(input)?;
    let (input, mut number) = map_res(digit1, str::parse)(input)?;
    if minus.is_some() {
        number *= -1;
    }
    Ok((input, Operation::Addx(number)))
}

fn noop(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("noop")(input)?;

    Ok((input, Operation::Noop))
}

fn operations(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, ops) = separated_list1(newline, alt((addx, noop)))(input)?;

    Ok((input, ops))
}

pub fn process_part1(input: &str) -> String {
    let (_, operations) = operations(input).unwrap(); // valid
    dbg!(&operations);
    "one".to_string()
}

pub fn process_part2(input: &str) -> String {
    "two".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "13140");
    }
}
