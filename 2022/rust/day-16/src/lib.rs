use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, line_ending, newline},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    tunnels: Vec<&'a str>,
}

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

fn parse_line(input: &str) -> IResult<&str, Valve> {
    let (input, (name, flow_rate, tunnels)) =
        tuple((parse_valve, parse_flow_rate, parse_tunnels))(input)?;

    Ok((
        input,
        Valve {
            name,
            flow_rate,
            tunnels,
        },
    ))
}

fn parse_valves(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(line_ending, parse_line)(input)
}

pub fn process_part1(input: &str) -> String {
    let (_input, valves) = parse_valves(input).unwrap();
    dbg!(valves);

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
