use itertools::Itertools;
use lending_iterator::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{cmp::Ordering, collections::HashSet};

#[derive(Eq, PartialEq, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_dir(&mut self, dir: Direction) {
        match dir {
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
            Direction::Up => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
        };
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point ({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, c) = alt((
        complete::char('L'),
        complete::char('R'),
        complete::char('U'),
        complete::char('D'),
    ))(input)?;

    let dir = match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        'U' => Direction::Up,
        'D' => Direction::Down,
        _ => unreachable!(),
    };

    Ok((input, dir))
}

fn moves(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, vecs) =
        separated_list1(newline, separated_pair(direction, tag(" "), complete::u32))(input)?;

    let vecs = vecs
        .iter()
        .flat_map(|(dir, repeat)| vec![*dir; *repeat as usize])
        .collect();

    Ok((input, vecs))
}

pub fn process_part1(input: &str) -> String {
    let (_, move_set) = moves(input).unwrap();
    let mut head = Pos::zero();
    let mut tail = Pos::zero();
    let mut tail_positions = HashSet::from([tail.clone()]);

    for dir in move_set.iter() {
        head.move_dir(*dir);
        let x_range = (&head.x - 1)..=(&head.x + 1);
        let y_range = (&head.y - 1)..=(&head.y + 1);

        let tail_is_connected = x_range
            .cartesian_product(y_range)
            .any(|(x, y)| Pos::new(x, y) == tail);

        if !tail_is_connected {
            let mut new_tail = head.clone();
            match dir {
                Direction::Left => new_tail.move_dir(Direction::Right),
                Direction::Right => new_tail.move_dir(Direction::Left),
                Direction::Up => new_tail.move_dir(Direction::Down),
                Direction::Down => new_tail.move_dir(Direction::Up),
            };
            tail = new_tail.clone();
            tail_positions.insert(new_tail);
        }
    }

    tail_positions.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, move_set) = moves(input).unwrap();
    let mut rope: Vec<Pos> = vec![Pos::zero(); 10];
    let mut tail_positions = HashSet::from([rope.last().unwrap().clone()]);

    for dir in move_set.iter() {
        rope[0].move_dir(*dir);
        let mut rope_windows = rope.windows_mut::<2>();
        // let mut last_head_move = dir.clone();
        while let Some([ref mut head, ref mut tail]) = rope_windows.next() {
            let x_range = (head.x - 1)..=(head.x + 1);
            let y_range = (head.y - 1)..=(head.y + 1);
            let head_cross_positions: Vec<Pos> = x_range
                .clone()
                .cartesian_product(y_range.clone())
                .map(|(x, y)| Pos::new(x, y))
                .collect();
            let follow_is_connected = head_cross_positions.clone().iter().any(|pos| *pos == *tail);

            if !follow_is_connected {
                // println!("{last_head_move:?}");
                // let mut new_follow = head.clone();
                if head.x == tail.x {
                    match head.y.cmp(&tail.y) {
                        Ordering::Greater => tail.y += 1,
                        Ordering::Less => tail.y -= 1,
                        Ordering::Equal => {}
                    };
                } else if head.y == tail.y {
                    match head.x.cmp(&tail.x) {
                        Ordering::Greater => tail.x += 1,
                        Ordering::Less => tail.x -= 1,
                        Ordering::Equal => {}
                    }
                } else {
                    // let head_cross_positions = [
                    //     (head.x - 1, head.y),
                    //     (head.x + 1, head.y),
                    //     (head.x, head.y - 1),
                    //     (head.x, head.y + 1),
                    // ];

                    let head_cross_positions =
                        x_range.cartesian_product(y_range).collect::<Vec<_>>();
                    let x_range = (tail.x - 1)..=(tail.x + 1);
                    let y_range = (tail.y - 1)..=(tail.y + 1);

                    let maybe_new_tail: Vec<Pos> = x_range
                        .cartesian_product(y_range)
                        .filter(|tuple| head_cross_positions.contains(tuple))
                        .map(|(x, y)| Pos::new(x, y))
                        .collect();
                    match maybe_new_tail.len() {
                        2 => {}
                        1 => {
                            *tail = maybe_new_tail[0];
                        }
                        _ => {
                            panic!("Unknown Tail Length")
                        }
                    }
                    // *tail = new_tail;
                }
            }
        }
        tail_positions.insert(rope.last().unwrap().clone());
        dbg!(dir, &rope);
    }

    tail_positions.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT_1), "13");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT_2), "36");
    }
}
