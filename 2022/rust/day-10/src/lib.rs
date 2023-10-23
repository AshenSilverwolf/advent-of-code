use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map_res, opt};
use nom::multi::separated_list1;
use nom::*;

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
    let key_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut key_values = vec![];
    let mut register: i32 = 1;

    let mut cycle: i32 = 0;
    for op in operations {
        cycle += 1;
        if key_cycles.contains(&cycle) {
            key_values.push((register, cycle));
        }
        if let Operation::Addx(num) = op {
            cycle += 1;
            if key_cycles.contains(&cycle) {
                key_values.push((register, cycle));
            }
            register += num;
        }
    }

    let output: i32 = key_values.iter().map(|(val, cycle)| val * cycle).sum();

    output.to_string()
}

pub fn process_part2(_input: &str) -> String {
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

    const OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "13140");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), OUTPUT.to_string());
    }
}
