use geo::{Coord, Line};
use intersect2d::{intersect, Intersection};
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MIN_BOUND: f64 = 0.;
const MAX_BOUND: f64 = 4_000_000.;

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

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

type Sensor = Pos;
type Beacon = Pos;
type Pair = (Sensor, Beacon);
type LineSeg = (Pos, Pos);

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> Vec<Pair> {
    let sep_str = r"Sensor at |, |: closest beacon is at ";
    let seperator = Regex::new(sep_str).expect("Invalid regex");
    let mut pairs: Vec<(Sensor, Beacon)> = vec![];
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
            pairs.push((sensor, beacon));
        }
    }

    pairs
}

fn generate_border_lines(sensor: &Sensor, range: i32) -> HashSet<LineSeg> {
    let mut lines: HashSet<LineSeg> = HashSet::new();

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

    lines.insert((east.clone(), south.clone()));
    lines.insert((south.clone(), west.clone()));
    lines.insert((west.clone(), north.clone()));
    lines.insert((north.clone(), east.clone()));

    lines
}

fn run_logic(pairs: Vec<Pair>) -> Pos {
    let mut output: Pos = Pos::zero();
    let mut lines: HashSet<LineSeg> = HashSet::new();
    let mut intersections: HashSet<Pos> = HashSet::new();
    for (sensor, beacon) in &pairs {
        let border_lines = generate_border_lines(&sensor, sensor.dist(&beacon));
        lines = lines.union(&border_lines).map(|ls| ls.to_owned()).collect();
    }

    let lines_iter1 = lines
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
    let lines_iter2 = lines_iter1.clone();

    for line1 in &lines_iter1 {
        for line2 in &lines_iter2 {
            if line1 == line2 {
                continue;
            }
            let intersection_point = intersect(&line1, &line2);
            if let Some(Intersection::Intersection(c)) = intersection_point {
                let within_bounds: bool =
                    MIN_BOUND <= c.x && c.x <= MAX_BOUND && MIN_BOUND <= c.y && c.y <= MAX_BOUND;
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
                for (sensor, beacon) in &pairs {
                    if point.dist(&sensor) <= sensor.dist(&beacon) {
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

    output
}

fn main() {
    let pairs = parse_input();
    let lost_beacon = run_logic(pairs);
    let tuning_frequency = lost_beacon.x as i64 * MAX_BOUND as i64 + lost_beacon.y as i64;
    println!("{tuning_frequency}");
}
