use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use std::cmp::Ordering;

#[derive(Debug, Eq)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Number(b)) => a.cmp(&vec![Self::Number(*b)]),
            (Self::Number(a), Self::List(b)) => vec![Self::Number(*a)].cmp(b),
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a == b,
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::List(a), Self::Number(b)) => a == &vec![Self::Number(*b)],
            (Self::Number(a), Self::List(b)) => &vec![Self::Number(*a)] == b,
        }
    }
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]")).map(Packet::List),
        complete::u32.map(Packet::Number),
    ))(input)
}

fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(|(p1, p2)| Pair {
            left: p1,
            right: p2,
        }),
    )(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, pair_list) = pairs(input).unwrap();
    pair_list
        .iter()
        .enumerate()
        .filter_map(|(index, Pair { left, right })| match left.cmp(right) {
            Ordering::Less => Some(index),
            Ordering::Equal => panic!("equal???"),
            Ordering::Greater => None,
        })
        .map(|i| i + 1)
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    "two".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "13");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "140");
    }
}
