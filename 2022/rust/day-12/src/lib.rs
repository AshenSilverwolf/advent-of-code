use nom::{
    character::complete::{alpha1, newline},
    multi::separated_list1,
    IResult, Parser,
};
use pathfinding::prelude::astar;

type Grid = Vec<Vec<i32>>;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn new() -> Pos {
        Pos(0, 0)
    }

    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, grid: &Grid) -> Vec<(Pos, u32)> {
        let &Pos(this_r, this_c) = self;
        [
            this_r.checked_sub(1).map(|diff| Pos(diff, this_c)),
            Some(Pos(this_r + 1, this_c)),
            this_c.checked_sub(1).map(|diff| Pos(this_r, diff)),
            Some(Pos(this_r, this_c + 1)),
        ]
        .into_iter()
        .flatten()
        .filter(|Pos(r, c)| {
            *r < grid.len() && *c < grid[0].len() && grid[*r][*c] - grid[this_r][this_c] <= 1
        })
        .map(|p| (p, 1))
        .collect()
    }
}

fn grid_start_end(input: &str) -> IResult<&str, (Grid, Pos, Pos)> {
    let (_, mut grid) = separated_list1(
        newline,
        alpha1.map(|chars: &str| {
            chars
                .chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => -1,
                    _ => c as i32 - 96,
                })
                .collect::<Vec<i32>>()
        }),
    )(input)?;

    let mut start = Pos::new();
    let mut end = Pos::new();
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, col) in row.iter_mut().enumerate() {
            if *col == 0 {
                start = Pos(r, c);
                *col = 1;
            } else if *col == -1 {
                end = Pos(r, c);
                *col = 26;
            }
        }
    }

    Ok((input, (grid, start, end)))
}

fn grid_with_multiple_starts(input: &str) -> IResult<&str, (Grid, Vec<Pos>, Pos)> {
    let (_, mut grid) = separated_list1(
        newline,
        alpha1.map(|chars: &str| {
            chars
                .chars()
                .map(|c| match c {
                    'S' => 1,
                    'E' => -1,
                    _ => c as i32 - 96,
                })
                .collect::<Vec<i32>>()
        }),
    )(input)?;

    let mut starts: Vec<Pos> = vec![];
    let mut end = Pos::new();
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, col) in row.iter_mut().enumerate() {
            if *col == 1 {
                starts.push(Pos(r, c));
            } else if *col == -1 {
                end = Pos(r, c);
                *col = 26;
            }
        }
    }

    Ok((input, (grid, starts, end)))
}

pub fn process_part1(input: &str) -> String {
    let (_, (grid, start, end)) = grid_start_end(input).unwrap();
    astar(
        &start,
        |p| p.successors(&grid),
        |p| p.distance(&end),
        |p| *p == end,
    )
    .unwrap()
    .1
    .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, (grid, starts, end)) = grid_with_multiple_starts(input).unwrap();

    starts
        .iter()
        .map(|start| {
            astar(
                start,
                |p| p.successors(&grid),
                |p| p.distance(&end),
                |p| *p == end,
            )
        })
        .filter_map(|res| res.map(|(_path, dist)| dist))
        .min()
        .unwrap()
        .to_string()

    // possible optimizations
    // - work backwards from the end to the nearest 'a'
    // 		- modify heuristic to calculate distance to nearest 'a' instead of single point
    // 		- modify success condition to match on any 'a'
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "31");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "29");
    }
}
