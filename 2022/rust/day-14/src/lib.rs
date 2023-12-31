use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::BTreeSet;

fn line(input: &str) -> IResult<&str, impl Iterator<Item = (u32, u32)>> {
    let (input, pairs) = separated_list1(
        tag(" -> "),
        separated_pair(complete::u32, complete::char(','), complete::u32),
    )(input)?;

    let result = pairs
        .into_iter()
        .tuple_windows()
        .flat_map(|((ax, ay), (bx, by))| {
            let x_min = ax.min(bx);
            let x_max = ax.max(bx);
            let x_range = x_min..=x_max;

            let y_min = ay.min(by);
            let y_max = ay.max(by);
            let y_range = y_min..=y_max;

            x_range.cartesian_product(y_range)
        });

    Ok((input, result))
}

fn rocks(input: &str) -> IResult<&str, BTreeSet<(u32, u32)>> {
    let (input, pairs) = separated_list1(line_ending, line)(input)?;
    let map: BTreeSet<(u32, u32)> = pairs.into_iter().flatten().collect();

    Ok((input, map))
}

pub fn process_part1(input: &str) -> String {
    let (_input, mut rocks) = rocks(input).unwrap();

    let mut rocks_vec: Vec<&(u32, u32)> = rocks.iter().collect();
    rocks_vec.sort_by(|a, b| a.1.cmp(&b.1));
    let lowest_rock = **rocks_vec.last().unwrap();
    drop(rocks_vec);

    let rock_count = rocks.len();
    let mut current_sand = (500, 0);
    loop {
        if current_sand.1 > lowest_rock.1 {
            break;
        }
        let down = (current_sand.0, current_sand.1 + 1);
        let down_left = (current_sand.0 - 1, current_sand.1 + 1);
        let down_right = (current_sand.0 + 1, current_sand.1 + 1);
        match (
            rocks.get(&down),
            rocks.get(&down_left),
            rocks.get(&down_right),
        ) {
            (None, _, _) => {
                current_sand = down;
            }
            (_, None, _) => {
                current_sand = down_left;
            }
            (_, _, None) => {
                current_sand = down_right;
            }
            (Some(_), Some(_), Some(_)) => {
                rocks.insert(current_sand);
                current_sand = (500, 0);
            }
        };
    }

    (rocks.len() - rock_count).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_input, mut rocks) = rocks(input).unwrap();

    let mut rocks_vec: Vec<&(u32, u32)> = rocks.iter().collect();
    rocks_vec.sort_by(|a, b| a.1.cmp(&b.1));
    let lowest_rock = **rocks_vec.last().unwrap();
    drop(rocks_vec);

    let rock_count = rocks.len();
    let mut current_sand = (500, 0);
    while rocks.get(&(500, 0)).is_none() {
        let down = (current_sand.0, current_sand.1 + 1);
        let down_left = (current_sand.0 - 1, current_sand.1 + 1);
        let down_right = (current_sand.0 + 1, current_sand.1 + 1);
        match (
            rocks.get(&down).or({
                if down.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            rocks.get(&down_left).or({
                if down_left.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            rocks.get(&down_right).or({
                if down_right.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
        ) {
            (None, _, _) => {
                current_sand = down;
            }
            (_, None, _) => {
                current_sand = down_left;
            }
            (_, _, None) => {
                current_sand = down_right;
            }
            (Some(_), Some(_), Some(_)) => {
                rocks.insert(current_sand);
                current_sand = (500, 0);
            }
        };
    }

    (rocks.len() - rock_count).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "24");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "93");
    }
}
