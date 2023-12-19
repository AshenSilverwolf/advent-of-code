#![feature(isqrt)]

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{newline, one_of},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use primes::factors;
use std::collections::BTreeMap;

type NodeLRMap<'a> = BTreeMap<&'a str, (&'a str, &'a str)>;

#[derive(Debug, Clone)]
enum Turn {
    Left,
    Right,
}

fn turns(input: &str) -> IResult<&str, Vec<Turn>> {
    many1(alt((
        value(Turn::Left, one_of("L")),
        value(Turn::Right, one_of("R")),
    )))(input)
}

fn take3(input: &str) -> IResult<&str, &str> {
    take(3usize)(input)
}

fn node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, origin) = take3(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, (left, right)) =
        delimited(tag("("), separated_pair(take3, tag(", "), take3), tag(")"))(input)?;

    let output = (origin, (left, right));

    Ok((input, output))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Turn>, NodeLRMap)> {
    let (input, (turns, nodes)) = separated_pair(
        turns,
        tuple((newline, newline)),
        separated_list1(newline, node),
    )(input)?;

    let nodes_map: BTreeMap<&str, (&str, &str)> = BTreeMap::from_iter(nodes);

    Ok((input, (turns, nodes_map)))
}

pub fn process_part1(input: &str) -> String {
    let (_, (turns, nodes_map)) = parse_input(input).expect("good things");

    let mut turns_iter = turns.iter().cycle();
    let mut steps: u32 = 0;
    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        let turn = turns_iter.next().expect("valid turn");
        let (left, right) = nodes_map.get(&current_node).expect("valid node");
        current_node = match turn {
            Turn::Left => left,
            Turn::Right => right,
        };
        steps += 1;
    }

    steps.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, (turns, nodes_map)) = parse_input(input).expect("good things");

    let turns_iter = turns.iter().cycle();
    let current_nodes: Vec<&str> = nodes_map
        .keys()
        .cloned()
        .filter(|s| s.ends_with('A'))
        .collect();
    dbg!(&current_nodes);

    current_nodes
        .iter()
        .map(|node| {
            let mut turns = turns_iter.clone();
            let mut steps = 0;
            let mut curr_node = *node;
            while !curr_node.ends_with('Z') {
                let turn = turns.next().expect("next turn");
                let (left, right) = nodes_map.get(curr_node).expect("valid left right");
                curr_node = match *turn {
                    Turn::Left => *left,
                    Turn::Right => *right,
                };
                steps += 1;
            }
            steps
        })
        .flat_map(factors)
        .map(|n| n as u128)
        .product::<u128>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_works() {
        let expected = String::from("6");
        let result = process_part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn part2_works() {
        let expected = String::from("6");
        let result = process_part2(INPUT_2);
        assert_eq!(expected, result);
    }

    #[test]
    fn prime_factorizations() {
        assert_eq!(vec![2, 3, 17], factors(102));
    }
}
