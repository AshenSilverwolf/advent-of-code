use nom::{
    IResult,
    combinator::map,
    multi::{many1, separated_list1},
    character::complete::{one_of, newline},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Pipe {
    Vert,
    Horiz,
    NE,
    NW,
    SE,
    SW,
}

impl Pipe {
    fn successors(&self) -> Vec<Direction> {
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

#[derive(Debug)]
enum Tile {
    Ground,
    Pipe(Pipe),
    Start,
}

impl Tile {
    fn imply_pipe(&self, (east: Tile, south: Tile, west: Tile, north: Tile)) -> Pipe {
        let con_e = match east {
            Tile::Pipe(Pipe::NW) => true,
            Tile::Pipe(Pipe::SW) => true,
            Tile::Pipe(Pipe::Horiz) => true,
            _ => false,
        };
        let con_s = match south {
            Tile::Pipe(Pipe::NW) => true,
            Tile::Pipe(Pipe::NE) => true,
            Tile::Pipe(Pipe::Vert) => true,
            _ => false,
        }; 
        let con_w = match west {
            Tile::Pipe(Pipe::NE) => true,
            Tile::Pipe(Pipe::SE) => true,
            Tile::Pipe(Pipe::Horiz) => true,
            _ => false,
        };
        let con_n = match north {
            Tile::Pipe(Pipe::SE) => true,
            Tile::Pipe(Pipe::SW) => true,
            Tile::Pipe(Pipe::Vert) => true,
            _ => false,
        };

        match (con_e, con_s, con_w, con_n) {
            (true, true, false, false) => Pipe::SE,
            (true, false, true, false) => Pipe::Horiz,
            (true, false, false, true) => Pipe::NE,
            (false, true, true, false) => Pipe::SW,
            (false, true, false, true) => Pipe::Vert,
            (false, false, true, true) => Pipe::NW
            _ => panic!("Invalid connections")
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

// you don't have to scan every tile to find the main loop
// just pick a direction from the start and stop when you return to the start
pub fn process_part1(input: &str) -> String {
    let (_, tile_grid) = parse_input(input).expect("valid input");
    dbg!(tile_grid);

    todo!()
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
