use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
enum SandState {
    Falling,
    Settled,
}

impl std::fmt::Display for SandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Falling => "Falling",
            Self::Settled => "Settled",
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Wall(Pos, Pos);

impl Wall {
    fn generate_pos_set(&self) -> HashSet<Pos> {
        let mut points: HashSet<Pos> = HashSet::new();
        let Wall(first, second) = self;
        let x_cmp = first.x.cmp(&second.x);
        let y_cmp = first.y.cmp(&second.y);

        if x_cmp == std::cmp::Ordering::Equal {
            match y_cmp {
                std::cmp::Ordering::Less => {
                    for i in first.y..=second.y {
                        points.insert(Pos { x: first.x, y: i });
                    }
                }
                std::cmp::Ordering::Greater => {
                    for i in second.y..=first.y {
                        points.insert(Pos { x: first.x, y: i });
                    }
                }
                std::cmp::Ordering::Equal => unreachable!(),
            };
        } else if y_cmp == std::cmp::Ordering::Equal {
            match x_cmp {
                std::cmp::Ordering::Less => {
                    for i in first.x..=second.x {
                        points.insert(Pos { x: i, y: first.y });
                    }
                }
                std::cmp::Ordering::Greater => {
                    for i in second.x..=first.x {
                        points.insert(Pos { x: i, y: first.y });
                    }
                }
                std::cmp::Ordering::Equal => unreachable!(),
            };
        } else {
            unreachable!();
        }

        points
    }
}

impl std::fmt::Display for Wall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Wall(first, second) = self;
        write!(f, "[{}, {}]", first, second)
    }
}

struct Sand {
    pos: Pos,
    state: SandState,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn pos_to_wall(pos_list: Vec<Pos>) -> HashSet<Wall> {
    let len: usize = pos_list.len();
    let mut walls: HashSet<Wall> = HashSet::new();

    for i in 1..len {
        walls.insert(Wall(pos_list[i - 1].clone(), pos_list[i].clone()));
    }

    walls
}

fn parse_input() -> HashSet<Wall> {
    let mut walls: HashSet<Wall> = HashSet::new();
    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.into_iter().flatten() {
            let mut wall_pos: Vec<Pos> = vec![];
            let coords = line.split(" -> ");
            for coord in coords {
                let pair: Vec<&str> = coord.split(',').collect();
                let x = pair[0].parse::<i32>().unwrap();
                let y = pair[1].parse::<i32>().unwrap();
                wall_pos.push(Pos { x, y });
            }

            // wonder if there's a better way to do this
            // feels non-idiomatic
            walls = walls
                .union(&pos_to_wall(wall_pos))
                .map(|x| x.to_owned())
                .collect();
        }
    }

    walls
}

fn generate_points_from_walls(walls: &HashSet<Wall>) -> HashSet<Pos> {
    let mut points: HashSet<Pos> = HashSet::new();
    for wall in walls {
        points = points
            .union(&wall.generate_pos_set())
            .map(|x| x.to_owned())
            .collect();
    }

    points
}

fn determine_lower_limit(walls: &HashSet<Wall>) -> i32 {
    let mut lowest: i32 = 0;
    for Wall(first, second) in walls {
        lowest = if first.y > lowest { first.y } else { lowest };
        lowest = if second.y > lowest { second.y } else { lowest };
    }

    lowest
}

fn run_logic(walls: HashSet<Wall>) -> i32 {
    let mut points: HashSet<Pos> = generate_points_from_walls(&walls);
    let lower_limit: i32 = determine_lower_limit(&walls);
    let source = Pos { x: 500, y: 0 };
    let mut num_settled: i32 = 0;

    while !points.contains(&source) {
        let mut sand: Sand = Sand {
            pos: source.clone(),
            state: SandState::Falling,
        };

        while sand.state == SandState::Falling {
            let bottom = Pos {
                x: sand.pos.x,
                y: sand.pos.y + 1,
            };
            let bottom_left = Pos {
                x: sand.pos.x - 1,
                y: sand.pos.y + 1,
            };
            let bottom_right = Pos {
                x: sand.pos.x + 1,
                y: sand.pos.y + 1,
            };
            if !points.contains(&bottom) {
                sand.pos.y += 1;
            } else if !points.contains(&bottom_left) {
                sand.pos.y += 1;
                sand.pos.x -= 1;
            } else if !points.contains(&bottom_right) {
                sand.pos.y += 1;
                sand.pos.x += 1;
            } else {
                points.insert(sand.pos.clone());
                sand.state = SandState::Settled;
            }

            if sand.pos.y > lower_limit {
                points.insert(sand.pos.clone());
                sand.state = SandState::Settled;
            }
        }

        num_settled += 1;
    }

    num_settled
}

fn main() {
    let walls = parse_input();
    let num_settled = run_logic(walls);
    println!("{num_settled}");
}
