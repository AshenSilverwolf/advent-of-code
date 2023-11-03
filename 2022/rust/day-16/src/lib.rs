use std::collections::BTreeMap;
use nom::{
    character::complete::{self, line_ending},
    sequence::preceded,
    bytes::complete::{tag, take},
    multi::separated_list1,
    branch::alt,
    IResult,
};

fn valve_name(input: &str) -> IResult<&str, &str> {
    preceded(tag("Valve "), take(2_usize))(input)
}

fn flow_rate(input: &str) -> IResult<&str, u32> {
    preceded(tag(" has flow rate="), complete::u32)(input)
}

fn tunnels(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        alt((
            tag("; tunnels lead to valves "), 
            tag("; tunnel leads to valve "),
        )),
        separated_list1(
            tag(", "),
            take(2_usize)
        )
    )(input)
}

fn line(input: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    let (input, name) = valve_name(input)?;
    let (input, flow) = flow_rate(input)?;
    let (input, tunnels) = tunnels(input)?;
    Ok((input, (name, flow, tunnels)))
}

fn parse_valves(input: &str) -> IResult<&str, (BTreeMap<&str, u32>, BTreeMap<&str, Vec<&str>>)> {
    let (input, result) = separated_list1(line_ending, line)(input)?;
    let mut valves: BTreeMap<&str, u32> = BTreeMap::new();
    let mut tunnels: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (name, flow, exit_tunnels) in result {
        valves.insert(name, flow);
        tunnels.insert(name, exit_tunnels);
    }

    Ok((input, (valves, tunnels)))
}

pub fn process_part1(input: &str) -> String {
    let (_input, (valves, tunnels)) = parse_valves(input).unwrap();
    dbg!(valves, tunnels);

    todo!("one")
}

pub fn process_part2(_input: &str) -> String {
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
    fn parser_works() {        
        let (input, name) = valve_name(INPUT).unwrap();
        assert_eq!(name, "AA");
        let (input, flow) = dbg!(flow_rate(input)).unwrap();
        assert_eq!(flow, 0_u32);
        let (input, tunnels) = tunnels(input).unwrap();
        assert_eq!(tunnels, vec!["DD", "II", "BB"]);
        let (input, (name, flow, tunnels)) = preceded(line_ending, line)(input).unwrap();
        assert_eq!(
            (name, flow, tunnels),
            ("BB", 13_u32, vec!["CC", "AA"]),
        );
    }

    #[test]
    #[ignore]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "1651");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "1651");
    }
}
