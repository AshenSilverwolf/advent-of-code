use nom::{
    IResult,
    character::complete::{one_of, newline},
    multi::{many1, separated_list1},
    combinator::value,
    sequence::{separated_pair, delimited, tuple},
    bytes::complete::{tag, take},
    branch::alt,
};
use std::collections::BTreeMap;

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
    let (input, (left, right)) = delimited(
        tag("("),
        separated_pair(
            take3,
            tag(", "),
            take3,
        ),
        tag(")"),
    )(input)?;

    let output = (origin, (left, right));

    Ok((input, output))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Turn>, BTreeMap<&str, (&str, &str)>)> {
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
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part1_works() {
        let expected = String::from("6");
        let result = process_part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let expected = String::from("");
        let result = process_part2(INPUT);
        assert_eq!(expected, result);
    }
}
