// pathfinding also has a Grid, but it doesn't meet our needs here
// https://docs.rs/pathfinding/4.3.1/pathfinding/grid/struct.Grid.html#
use pathfinding::prelude::{astar, bfs, dfs, dijkstra};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn new() -> Pos {
        Pos(0, 0)
    }

    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, grid: &Vec<Vec<i32>>) -> Vec<(Pos, u32)> {
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> (Pos, Pos, Vec<Vec<i32>>) {
    let mut start: Pos = Pos::new();
    let mut end: Pos = Pos::new();
    let mut grid: Vec<Vec<i32>> = vec![];
    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.into_iter().flatten() {
            let row: Vec<i32> = line
                .chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => -1,
                    _ => c as i32 - 96,
                })
                .collect();
            grid.push(row);
        }
    }

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 0 {
                start = Pos(r, c);
                grid[r][c] = 1;
            } else if grid[r][c] == -1 {
                end = Pos(r, c);
                grid[r][c] = 26;
            }
        }
    }

    (start, end, grid)
}

fn main() {
    let (start, end, grid) = parse_input();

    println!("start: {:?}\nend: {:?}", start, end);

    let result = astar(
        &start,
        |p| p.successors(&grid),
        |p| p.distance(&end),
        |p| *p == end,
    );

    println!("{}", result.unwrap().1);
}
