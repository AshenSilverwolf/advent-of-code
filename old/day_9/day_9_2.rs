use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NUM_KNOTS: usize = 10;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    iterations: i32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    fn is_adjacent(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn update_pos(&mut self, direction: &Direction) {
        match direction {
            &Direction::Up => self.y += 1,
            &Direction::Down => self.y -= 1,
            &Direction::Left => self.x -= 1,
            &Direction::Right => self.x += 1,
        };
    }

    fn follow(&mut self, other: &Point) {
        if other.x - self.x > 0 && other.y - self.y > 0 {
            // Top-Right
            self.update_pos(&Direction::Up);
            self.update_pos(&Direction::Right);
        } else if other.x - self.x < 0 && other.y - self.y > 0 {
            // Top-Left
            self.update_pos(&Direction::Up);
            self.update_pos(&Direction::Left);
        } else if other.x - self.x < 0 && other.y - self.y < 0 {
            // Bottom-Left
            self.update_pos(&Direction::Down);
            self.update_pos(&Direction::Left);
        } else if other.x - self.x > 0 && other.y - self.y < 0 {
            // Bottom-Right
            self.update_pos(&Direction::Down);
            self.update_pos(&Direction::Right);
        } else if other.x - self.x > 0 && other.y == self.y {
            // Right
            self.update_pos(&Direction::Right);
        } else if other.x - self.x < 0 && other.y == self.y {
            // Left
            self.update_pos(&Direction::Left);
        } else if other.x == self.x && other.y - self.y > 0 {
            // Top
            self.update_pos(&Direction::Up);
        } else if other.x == self.x && other.y - self.y < 0 {
            // Bottom
            self.update_pos(&Direction::Down);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];

    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.into_iter().flatten() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let curr_command = Command {
                direction: match tokens[0] {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => todo!(),
                },
                iterations: tokens[1].parse().unwrap(),
            };
            commands.push(curr_command);
        }
    }

    commands
}

fn init_point_vec(size: usize) -> Vec<Point> {
    let mut knots: Vec<Point> = vec![];

    for _ in 0..size {
        knots.push(Point::new());
    }

    knots
}

// Issues with mutable/immutable references on knots
// fn run_logic(commands: Vec<Command>) -> usize {
//     let mut unique_tail_points: HashSet<Point> = HashSet::new();
//     let start = Point::new();
//     let mut knots: Vec<Point> = init_point_vec(NUM_KNOTS);

//     unique_tail_points.insert(start.clone());

//     for command in commands {
//         for _ in 0..command.iterations {
//             knots[0].update_pos(&command.direction);
//             for i in 1..=knots.len() {
//                 if !knots[i].is_adjacent(&knots[i-1].clone()) {
//                     knots[i].follow(&knots[i-1].clone());
//                 }
//                 unique_tail_points.insert(knots[9].clone());
//             }
//         }
//     }

//     unique_tail_points.len()
// }

// seems like you can have mutable and immutable references together
// with the caveat that you have to do all of type A, and then all of type B
// seems you can't do type A, then type B, then type A again; not allowed
fn run_logic(commands: Vec<Command>) -> usize {
    let mut unique_tail_points: HashSet<Point> = HashSet::new();
    let start = Point::new();
    let mut knots: Vec<Point> = init_point_vec(NUM_KNOTS);

    unique_tail_points.insert(start.clone());

    for command in commands {
        for _ in 0..command.iterations {
            knots[0].update_pos(&command.direction);
            for i in 1..knots.len() {
                let leader = knots[i - 1].clone();
                if !knots[i].is_adjacent(&leader) {
                    knots[i].follow(&leader);
                }
                unique_tail_points.insert(knots[9].clone());
            }
        }
    }

    unique_tail_points.len()
}

fn main() {
    let commands = parse_input();
    let unique_points = run_logic(commands);
    println!("{}", unique_points);
}
