use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

    fn points_within_range(&self, range: i32) -> HashSet<Pos> {
        let mut output: HashSet<Pos> = HashSet::new();
        for y in self.y - range..=self.y + range {
            for x in self.x - range..=self.x + range {
                output.insert(Pos { x, y });
            }
        }

        output.retain(|pos| self.dist(pos) <= range);

        for item in output.clone() {
            println!("{item}: {0}", item.dist(self));
        }

        output
    }
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

fn run_logic(pairs: Vec<Pair>) -> i32 {
    let mut row_2mil: HashSet<Pos> = HashSet::new();

    for (sensor, beacon) in pairs {
        let dist = sensor.dist(&beacon);
    }

    0
}

fn main() {
    let (pairs, _sensors, _beacons) = parse_input();
    // let _num_covered = run_logic(pairs);
    let test1 = Pos { x: 10, y: 10 };
    println!("{}", test1.points_within_range(3).len());
}
