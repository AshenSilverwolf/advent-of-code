use geo::{Coord, Line};
use intersect2d::{intersect, Intersection};
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
    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

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

fn generate_border_lines(sensor: &Sensor, range: i32) -> BTreeSet<(Pos, Pos)> {
    let east = Pos {
        x: sensor.x + range + 1,
        y: sensor.y,
    };
    let south = Pos {
        x: sensor.x,
        y: sensor.y - range - 1,
    };
    let west = Pos {
        x: sensor.x - range - 1,
        y: sensor.y,
    };
    let north = Pos {
        x: sensor.x,
        y: sensor.y + range + 1,
    };

    BTreeSet::from([
        (east.clone(), south.clone()),
        (south.clone(), west.clone()),
        (west.clone(), north.clone()),
        (north.clone(), east.clone()),
    ])
}

pub fn process_part2(input: &str, search_space: f64) -> String {
    let (_, sensor_beacon_map) = sensor_beacon_map(input).unwrap();
    let mut output: Pos = Pos::zero();
    let mut lines: BTreeSet<(Pos, Pos)> = BTreeSet::new();
    let mut intersections: BTreeSet<Pos> = BTreeSet::new();
    for (sensor, beacon) in &sensor_beacon_map {
        let border_lines = generate_border_lines(sensor, sensor.distance(beacon));
        lines = lines.union(&border_lines).map(|ls| ls.to_owned()).collect();
    }

    let lines_vec1 = lines
        .iter()
        .map(|(first, second)| {
            let mut coord1 = Coord::zero();
            let mut coord2 = Coord::zero();
            coord1.x = first.x as f64;
            coord1.y = first.y as f64;
            coord2.x = second.x as f64;
            coord2.y = second.y as f64;
            Line::new(coord1, coord2)
        })
        .collect::<Vec<Line>>();
    let lines_vec2 = lines_vec1.clone();

    for line1 in &lines_vec1 {
        for line2 in &lines_vec2 {
            if line1 == line2 {
                continue;
            }
            let intersection_point = intersect(line1, line2);
            if let Some(Intersection::Intersection(c)) = intersection_point {
                let within_bounds: bool =
                    0. <= c.x && c.x <= search_space && 0. <= c.y && c.y <= search_space;
                if within_bounds {
                    intersections.insert(Pos {
                        x: c.x as i32,
                        y: c.y as i32,
                    });
                }
            } else {
                continue;
            }

            for point in &intersections {
                let mut covered = false;
                for (sensor, beacon) in &sensor_beacon_map {
                    if point.distance(sensor) <= sensor.distance(beacon) {
                        covered = true;
                        break;
                    }
                }
                if !covered {
                    output = point.clone();
                    break;
                }
            }
        }
    }

    dbg!(&output);

    (output.x as i64 * 4_000_000 + output.y as i64).to_string()
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
    fn part2_works() {
        assert_eq!(process_part2(INPUT, 20.), "56000011");
    }
}
