use nom::{
    branch::alt,
    bytes::complete::tag,
    IResult, Parser,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
};

#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(
            tag("["),
            separated_list0(
                tag(","),
                packet,
            ),
            tag("]"),
        ).map(|vec| Packet::List(vec)),
        complete::u32.map(|num| Packet::Number(num)),
    ))(input)
}

fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(
            |(p1, p2)| Pair { left: p1, right: p2 }
        )
    )(input)
}

pub fn process_part1(input: &str) -> String {
    let(_, pair_list) = pairs(input).unwrap();
    // dbg!(pair_list);
    pair_list
        .iter()
        .enumerate()
        .map(|(index, Pair { left, right })| (index+1, left.cmp(right)))
        .filter(|(_, less_than)| less_than)
        .sum::<u32>()
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
