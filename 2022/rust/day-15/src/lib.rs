use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

type Sensor = Pos;
type Beacon = Pos;

fn position(input: &str) -> IResult<&str, Pos> {
    let (input, (x, y)) = separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)?;

    Ok((input, Pos { x, y }))
}

fn line(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    let (input, pair) = tuple((
        preceded(tag("Sensor at "), position),
        preceded(tag(": closest beacon is at "), position),
    ))(input)?;

    Ok((input, pair))
}

fn sensor_beacon_map(input: &str) -> IResult<&str, BTreeMap<Sensor, Beacon>> {
    let (input, list) = separated_list1(line_ending, line)(input)?;

    Ok((
        input,
        list.into_iter().collect::<BTreeMap<Sensor, Beacon>>(),
    ))
}

fn determine_boundaries(pairs: &BTreeMap<Sensor, Beacon>, target_row: i32) -> (i32, i32) {
    let mut left_bound = i32::MAX;
    let mut right_bound = i32::MIN;

    for (sensor, beacon) in pairs.iter() {
        let distance_to_beacon = sensor.distance(beacon);
        let distance_to_target_row = sensor.distance(&Pos {
            x: sensor.x,
            y: target_row,
        });
        let x_range = distance_to_beacon - distance_to_target_row;
        let left_x = sensor.x - x_range;
        let right_x = sensor.x + x_range;

        left_bound = left_bound.min(left_x);
        right_bound = right_bound.max(right_x);
    }

    (left_bound, right_bound)
}

pub fn process_part1(input: &str, target_row: i32) -> String {
    let (_, sensor_beacon_map) = sensor_beacon_map(input).unwrap();
    let (left_bound, right_bound) = determine_boundaries(&sensor_beacon_map, target_row);
    let mut covered = 0;

    for x in left_bound..=right_bound {
        let curr_point = Pos { x, y: target_row };

        if sensor_beacon_map
            .values()
            .collect::<BTreeSet<&Pos>>()
            .contains(&curr_point)
        {
            continue;
        }

        for (sensor, beacon) in &sensor_beacon_map {
            if sensor.distance(&curr_point) <= sensor.distance(beacon) {
                covered += 1;
                break;
            }
        }
    }

    covered.to_string()
}

pub fn process_part2(_input: &str) -> String {
    todo!("two")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT, 10), "26");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "56000011");
    }
}
