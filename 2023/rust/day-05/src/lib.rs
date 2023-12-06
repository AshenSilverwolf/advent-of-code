use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, none_of},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::collections::BTreeMap;

fn seeds(input: &str) -> IResult<&str, Vec<i64>> {
    preceded(tag("seeds: "), separated_list1(tag(" "), complete::i64))(input)
}

fn seeds_by_range(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, seed_ranges) = preceded(
        tag("seeds: "),
        separated_list1(
            tag(" "),
            separated_pair(complete::i64, tag(" "), complete::i64),
        ),
    )(input)?;

    let output = seed_ranges
        .iter()
        .flat_map(|(seed, range)| ((*seed)..(*seed + *range)).collect::<Vec<i64>>())
        .collect();

    Ok((input, output))
}

fn preamble(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((newline, newline, many1(none_of("\n")), newline))(input)?;

    Ok((input, ()))
}

fn a_to_b_map(input: &str) -> IResult<&str, BTreeMap<(i64, i64), i64>> {
    let (input, ranges) = separated_list1(
        newline,
        tuple((
            complete::i64,
            preceded(tag(" "), complete::i64),
            preceded(tag(" "), complete::i64),
        )),
    )(input)?;

    let mut a_b_map: BTreeMap<(i64, i64), i64> = BTreeMap::new();
    for (dst, src, len) in ranges {
        let dif = dst - src;
        let hi = src + len - 1;
        a_b_map.insert((src, hi), dif);
    }

    Ok((input, a_b_map))
}

pub fn process_part1(input: &str) -> String {
    let (input, seeds) = seeds(input).expect("valid seeds Vec");
    let (input, seed_soil_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid seed soil map");
    let (input, soil_fertilizer_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid soil fertilizer map");
    let (input, fertilizer_water_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid fertilizer soil map");
    let (input, water_light_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid water light map");
    let (input, light_temperature_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid light temperature map");
    let (input, temperature_humidity_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid temperature humidity map");
    let (input, humidity_location_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid humidity location map");
    assert!(input.is_empty());

    let maps = vec![
        seed_soil_map,
        soil_fertilizer_map,
        fertilizer_water_map,
        water_light_map,
        light_temperature_map,
        temperature_humidity_map,
        humidity_location_map,
    ];

    seeds
        .iter()
        .map(|seed| {
            let mut val = *seed;
            for map in maps.clone() {
                for ((lo, hi), dif) in map {
                    if lo <= val && val <= hi {
                        val += dif;
                        break;
                    }
                }
            }

            val
        })
        .min()
        .expect("some value")
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, seeds) = seeds_by_range(input).expect("valid seeds Vec");
    let (input, seed_soil_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid seed soil map");
    let (input, soil_fertilizer_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid soil fertilizer map");
    let (input, fertilizer_water_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid fertilizer soil map");
    let (input, water_light_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid water light map");
    let (input, light_temperature_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid light temperature map");
    let (input, temperature_humidity_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid temperature humidity map");
    let (input, humidity_location_map) =
        preceded(preamble, a_to_b_map)(input).expect("valid humidity location map");
    assert!(input.is_empty());

    let maps = vec![
        seed_soil_map,
        soil_fertilizer_map,
        fertilizer_water_map,
        water_light_map,
        light_temperature_map,
        temperature_humidity_map,
        humidity_location_map,
    ];

    seeds
        .iter()
        .map(|seed| {
            let mut val = *seed;
            for map in maps.clone() {
                for ((lo, hi), dif) in map {
                    if lo <= val && val <= hi {
                        val += dif;
                        break;
                    }
                }
            }

            val
        })
        .min()
        .expect("some value")
        .to_string()
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
        let expected = String::from("35");
        let result = process_part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    fn part2_works() {
        let expected = String::from("46");
        let result = process_part2(INPUT);
        assert_eq!(expected, result);
    }
}
