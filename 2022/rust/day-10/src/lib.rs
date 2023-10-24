use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    *,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Add(i32),
    Noop,
    Empty,
}
use nom::sequence::preceded;
use Operation::*;

fn addx(input: &str) -> IResult<&str, Operation> {
    let (input, num) = preceded(tag("addx "), complete::i32)(input)?;
    Ok((input, Add(num)))
}

fn noop(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Noop))
}

fn operations(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, ops) = separated_list1(newline, alt((addx, noop)))(input)?;
    Ok((input, ops))
}

pub fn process_part1(input: &str) -> String {
    let (_, operations) = operations(input).unwrap();
    let key_cycles = [20, 60, 100, 140, 180, 220];
    let mut key_cycle_values: HashMap<i32, i32> = HashMap::new();
    let mut register: i32 = 1;

    let mut cycle: i32 = 0;
    for op in operations {
        cycle += 1;
        if key_cycles.contains(&cycle) {
            key_cycle_values.insert(cycle, register);
        }
        if let Add(num) = op {
            cycle += 1;
            if key_cycles.contains(&cycle) {
                key_cycle_values.insert(cycle, register);
            }
            register += num;
        }
        if cycle > 220 {
            break;
        }
    }

    key_cycle_values
        .iter()
        .map(|(cycle, register)| cycle * register)
        .sum::<i32>()
        .to_string()
}

fn draw(crt_display: &mut [Vec<char>], cycle: &i32, register: &i32) {
    let pixels = (register - 1)..(register + 1);
    let row = (cycle / 40) as usize;
    let col = (cycle % 40) as usize;
    dbg!(format!("{}, {}, {}", row, col, cycle));
    crt_display[row].push(match pixels.contains(cycle) {
        true => '#',
        false => '.',
    });
}

pub fn process_part2(input: &str) -> String {
    let mut crt_display: Vec<Vec<char>> = Vec::with_capacity(6);
    for _ in 0..6 {
        crt_display.push(Vec::with_capacity(40));
    }
    let operations = operations(input).unwrap().1;
    let mut ops_iter = operations.into_iter().peekable();
    let mut register: i32 = 1;
    let mut cycle = 0;
    let mut curr_op = ops_iter.next().unwrap();
    let mut last_op = Empty;

    while cycle < 240 {
        //* If new op, Begin execution of operation
        // if curr_op != last_op {
        //     // match curr_op {
        //     //     Add(num) => {}
        //     //     _ => {}
        //     // }
        // }

        //* Draw on CRT
        draw(&mut crt_display, &cycle, &register);

        //* increment the cycle; finish execution of operation
        cycle += 1;
        let second_cycle = last_op == curr_op;
        last_op = curr_op.clone();
        if let Noop = curr_op {
            if let Some(new_op) = ops_iter.next() {
                curr_op = new_op;
            }
        } else if second_cycle {
            let Add(x) = curr_op else { unreachable!() };
            register += x;
            if let Some(new_op) = ops_iter.next() {
                curr_op = new_op;
            }
        }
    }

    crt_display
        .iter()
        .map(|row| row.iter().collect::<String>())
        .fold(String::new(), |acc, elem| acc + &elem + "\n")
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
    // #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), OUTPUT.to_string());
    }
}
