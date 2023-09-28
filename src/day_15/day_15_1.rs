use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const ROW_NUM: i32 = 2_000_000;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Pos {
    fn dist(&self, other: &Pos) -> i32 {
        i32::abs(self.x - other.x) + i32::abs(self.y - other.y)
    }

    // fn points_within_range(&self, range: i32) -> HashSet<Pos> {
    //     let mut output: HashSet<Pos> = HashSet::new();
    //     // very inefficient method of generating points
    //     // to generate a square and remove the corners out of range
    //     // change to generate only correct points
    //     for y in self.y - range..=self.y + range {
    //         for x in self.x - range..=self.x + range {
    //             output.insert(Pos { x, y });
    //         }
    //     }

    //     output.retain(|pos| self.dist(pos) <= range);

    //     output
    // }
}

type Sensor = Pos;
type Beacon = Pos;
type Pair = (Sensor, Beacon);

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> (Vec<Pair>, HashSet<Sensor>, HashSet<Beacon>) {
    let sep_str = r"Sensor at |, |: closest beacon is at ";
    let seperator = Regex::new(sep_str).expect("Invalid regex");
    let mut pairs: Vec<(Sensor, Beacon)> = vec![];
    let mut sensors: HashSet<Sensor> = HashSet::new();
    let mut beacons: HashSet<Beacon> = HashSet::new();
    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.into_iter().flatten() {
            let splits: Vec<i32> = seperator
                .split(line.as_ref())
                .filter(|x| !x.is_empty())
                .map(|x| x.get(2..).unwrap().parse::<i32>().unwrap())
                .collect();
            let sensor = Sensor {
                x: splits[0],
                y: splits[1],
            };
            let beacon = Beacon {
                x: splits[2],
                y: splits[3],
            };
            sensors.insert(sensor.clone());
            beacons.insert(beacon.clone());
            pairs.push((sensor, beacon));
        }
    }

    (pairs, sensors, beacons)
}

fn determine_boundary_values(pairs: &Vec<Pair>) -> (i32, i32) {
    let mut left_bound: i32 = i32::MAX;
    let mut right_bound: i32 = i32::MIN;

    for (sensor, beacon) in pairs {
        let dist_b = sensor.dist(beacon);
        let dist_y = sensor.dist(&Pos {
            x: sensor.x,
            y: ROW_NUM,
        });
        let d_x = dist_b - dist_y;
        let left_x = sensor.x - d_x;
        let right_x = sensor.x + d_x;

        left_bound = if left_x < left_bound {
            left_x
        } else {
            left_bound
        };
        right_bound = if right_x > right_bound {
            right_x
        } else {
            right_bound
        };
    }

    (left_bound, right_bound)
}

fn run_logic(pairs: Vec<Pair>) -> i32 {
    let (left_bound, right_bound) = determine_boundary_values(&pairs);
    let mut covered = 0;

    for x in left_bound..=right_bound {
        let curr_point = Pos { x, y: ROW_NUM };

        let mut is_beacon = false;
        for (_, beacon) in &pairs {
            if curr_point == *beacon {
                is_beacon = true;
                break;
            }
        }

        if is_beacon {
            continue;
        }

        for (sensor, beacon) in &pairs {
            if sensor.dist(&curr_point) <= sensor.dist(beacon) {
                covered += 1;
                break;
            }
        }
    }

    covered
}

fn main() {
    let (pairs, _sensors, _beacons) = parse_input();
    let num_covered = run_logic(pairs);
    println!("{num_covered}");
}
