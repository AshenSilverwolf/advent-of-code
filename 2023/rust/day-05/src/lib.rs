use std::collections::BTreeMap;
use nom::{
    multi::separated_list1,
    bytes::complete::tag,
    character::{is_newline, complete::{self, newline}},
    sequence::{preceded, tuple},
    IResult,
};

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), separated_list1(tag(" "), complete::u64))(input)
}

fn preamble(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((newline, newline, take_till(is_newline), newline))(input)?;

    Ok((input, ()))
}

fn a_to_b_map(input: &str) -> IResult<&str, BTreeMap<(u64, u64), u64>> {
    let (input, ranges) = separated_list1(
        newline,
        tuple((
            complete::u64,
            preceded(tag(" "), complete::u64),
            preceded(tag(" "), complete::u64),
        )),
    )(input)?;

    let mut a_b_map: BTreeMap<(u64, u64), u64> = BTreeMap::new();
    for (src, dst, len) in ranges {
        let dif = dst - src;
        for hi = src + len - 1;
        a_b_map.insert((src, hi), dif);
    }

    Ok((input, a_b_map))
}

pub fn process_part1(input: &str) -> String {
    let seeds: Vec<u64> = seeds(input).expect("valid seeds Vec");
    let (input, seed_soil_map): BTreeMap<(u64, u64), u64> =
        preceded(
            preamble,
            a_to_b_map,
        )(input).expect("valid seed soil map");
    let (input, soil_fertilizer_map): BTreeMap<(u64, u64), u64> =
        preceded(
            preamble,
            a_to_b_map,
        )(input).expect("valid soil fertilizer map");
    let (input, fertilizer_water_map): BTreeMap<(u64, u64), u64> =
        preceded(
            preamble,
            a_to_b_map,
        )(input).expect("valid fertilizer soil map");
    let (input, water_light_map): BTreeMap<(u64, u64), u64> =
        preceded(
            preamble,
            a_to_b_map,
        )(input).expect("valid water light map");
    let (input, light_temperature_map): BTreeMap<(u64, u64), u64> =
        preceded(
            preamble,
            a_to_b_map,
        )(input).expect("valid light temperature map");
    let (input, temperature_humidity_map): BTreeMap<(u64, u64), u64> =
        preceded(
            preamble,
            a_to_b_map,
        )(input).expect("valid temperature humidity map");
    let (input, humidity_location_map): BTreeMap<(u64, u64), u64> =
        preceded(
            preamble,
            a_to_b_map,
        )(input).expect("valid humidity location map");
}

pub fn process_part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_works() {
        let expected = String::from("");
        let result = process_part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn part2_works() {
        let expected = String::from("");
        let result = process_part2(INPUT);
        assert_eq!(expected, result);
    }
}
