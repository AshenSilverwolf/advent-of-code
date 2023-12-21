use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Pos {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn flip(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pipe {
    Vert,
    Horiz,
    NE,
    NW,
    SE,
    SW,
}

impl Pipe {
    fn connections(&self) -> Vec<Direction> {
        match *self {
            Pipe::Vert => vec![Direction::Up, Direction::Down],
            Pipe::Horiz => vec![Direction::Left, Direction::Right],
            Pipe::NE => vec![Direction::Up, Direction::Right],
            Pipe::NW => vec![Direction::Up, Direction::Left],
            Pipe::SE => vec![Direction::Down, Direction::Right],
            Pipe::SW => vec![Direction::Down, Direction::Left],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Ground,
    Pipe(Pipe),
    Start,
    NonExistent,
}

impl Tile {
    fn imply_pipe((east, south, west, north): (Tile, Tile, Tile, Tile)) -> Pipe {
        let con_e = matches!(
            east,
            Tile::Pipe(Pipe::NW) | Tile::Pipe(Pipe::SW) | Tile::Pipe(Pipe::Horiz)
        );
        let con_s = matches!(
            south,
            Tile::Pipe(Pipe::NW) | Tile::Pipe(Pipe::NE) | Tile::Pipe(Pipe::Vert)
        );
        let con_w = matches!(
            west,
            Tile::Pipe(Pipe::NE) | Tile::Pipe(Pipe::SE) | Tile::Pipe(Pipe::Horiz)
        );
        let con_n = matches!(
            north,
            Tile::Pipe(Pipe::SE) | Tile::Pipe(Pipe::SW) | Tile::Pipe(Pipe::Vert)
        );

        match (con_e, con_s, con_w, con_n) {
            (true, true, false, false) => Pipe::SE,
            (true, false, true, false) => Pipe::Horiz,
            (true, false, false, true) => Pipe::NE,
            (false, true, true, false) => Pipe::SW,
            (false, true, false, true) => Pipe::Vert,
            (false, false, true, true) => Pipe::NW,
            _ => panic!("Invalid connections"),
        }
    }
}

fn row(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(map(one_of("-|LJ7F.S"), |c| match c {
        '|' => Tile::Pipe(Pipe::Vert),
        '-' => Tile::Pipe(Pipe::Horiz),
        'L' => Tile::Pipe(Pipe::NE),
        'J' => Tile::Pipe(Pipe::NW),
        'F' => Tile::Pipe(Pipe::SE),
        '7' => Tile::Pipe(Pipe::SW),
        '.' => Tile::Ground,
        'S' => Tile::Start,
        other => panic!("unexpected tile: {}", other),
    }))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(newline, row)(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, tile_grid) = parse_input(input).expect("valid input");
    let mut start_pos = Pos { x: 0, y: 0 };

    for row in 0..tile_grid.len() {
        for col in 0..tile_grid[0].len() {
            if tile_grid[row][col] == Tile::Start {
                start_pos = Pos { x: col, y: row };
            }
        }
    }

    let mut num_moves = 0;
    let mut curr_pos = start_pos.clone();
    let east = if start_pos.x < tile_grid[0].len() {
        tile_grid[start_pos.y][start_pos.x + 1].clone()
    } else {
        Tile::NonExistent
    };
    let south = if start_pos.y < tile_grid.len() {
        tile_grid[start_pos.y][start_pos.x + 1].clone()
    } else {
        Tile::NonExistent
    };
    let west = if start_pos.x > 0 {
        tile_grid[start_pos.y][start_pos.x + 1].clone()
    } else {
        Tile::NonExistent
    };
    let north = if start_pos.y > 0 {
        tile_grid[start_pos.y][start_pos.x + 1].clone()
    } else {
        Tile::NonExistent
    };
    let start_neighbors = (east, south, west, north);

    let binding = Tile::imply_pipe(start_neighbors).connections();
    let mut next_move = binding
        .iter()
        .min()
        .expect("random direction of flow");

    match next_move {
        Direction::Up => curr_pos.y -= 1,
        Direction::Down => curr_pos.y += 1,
        Direction::Right => curr_pos.x += 1,
        Direction::Left => curr_pos.x -= 1,
    };
    let mut prev_move = next_move.clone();
    num_moves += 1;
    
    while curr_pos != start_pos {
        let p =  if let Tile::Pipe(p) = tile_grid[curr_pos.y][curr_pos.x].clone() { p } else { panic!("uhoh") };
        let binding = p.connections();
        next_move = binding
            .iter()
            .filter(|d| **d != prev_move.flip())
            .next()
            .expect("forward move");
        prev_move = next_move.clone();
        match next_move {
            Direction::Up => curr_pos.y -= 1,
            Direction::Down => curr_pos.y += 1,
            Direction::Right => curr_pos.x += 1,
            Direction::Left => curr_pos.x -= 1,
        };
        num_moves += 1;
    }
    
    (num_moves / 2).to_string()
}

pub fn process_part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn part1_works() {
        let expected = String::from("8");
        let result = process_part1(INPUT);
        assert_eq!(expected, result);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let expected = String::from("");
        let result = process_part2(INPUT);
        assert_eq!(expected, result);
    }
}
