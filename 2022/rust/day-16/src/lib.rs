use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

fn parse_valve(input: &str) -> IResult<&str, &str> {
    preceded(tag("Valve "), take(2_usize))(input)
}

fn parse_flow_rate(input: &str) -> IResult<&str, u32> {
    preceded(tag(" has flow rate="), complete::u32)(input)
}

fn parse_tunnels(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), take(2_usize)),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    let (input, (name, flow_rate, tunnels)) =
        tuple((parse_valve, parse_flow_rate, parse_tunnels))(input)?;

    Ok((input, (name, flow_rate, tunnels)))
}

fn parse_valves(input: &str) -> IResult<&str, (BTreeMap<&str, u32>, BTreeMap<&str, Vec<&str>>)> {
    let (input, lines) = separated_list1(line_ending, parse_line)(input)?;
    let mut valves: BTreeMap<&str, u32> = BTreeMap::new();
    let mut tunnels: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (name, flow, tunnels_list) in lines {
        valves.insert(name, flow);
        tunnels.insert(name, tunnels_list);
    }

    Ok((input, (valves, tunnels)))
}

pub fn process_part1(input: &str) -> String {
    let (_input, (valves, tunnels)) = parse_valves(input).unwrap();
    dbg!(valves, tunnels);

    todo!("process_part1")
}

pub fn process_part2(input: &str) -> String {
    todo!("two")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "1651");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "1651");
    }
}
