use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::HashSet;
// Note: HashSet requires that if k1 == k2, then hash(k1) == hash(k2).
// this could be difficult to implement with our GalacticPair equality logic

#[derive(Debug, Clone)]
struct GalacticPair(Pos, Pos);

impl PartialEq for GalacticPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Space,
    Galaxy,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(
        newline,
        many1(map(one_of(".#"), |c: char| match c {
            '.' => Tile::Space,
            '#' => Tile::Galaxy,
            _ => panic!("invalid input"),
        })),
    )(input)
}

fn isolate_column(galactic_map: &Vec<Vec<Tile>>, col_index: usize) -> impl Iterator<Item = Tile> {
    if col_index >= galactic_map.len() {
        panic!("col_index exceeds max size of map: {col_index}");
    }
    let col: Vec<Tile> = galactic_map
        .iter()
        .map(|row| row[col_index].clone())
        .collect();
    col.into_iter()
}

pub fn process_part1(input: &str) -> String {
    let (_, galaxy_map) = parse_input(input).expect("valid output");

    // create set of all unique pairs of galaxies
    let galaxies: HashSet<Pos> = galaxy_map
        .iter()
        .enumerate()
        .flat_map(|(y_index, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, t)| t == &&Tile::Galaxy)
                .map(|(x_index, _)| Pos {
                    x: x_index,
                    y: y_index,
                })
                .collect::<Vec<Pos>>()
        })
        .collect();
    dbg!(&galaxies);
    let galaxy_pairs: HashSet<GalacticPair>;
    // find distance between all pairs
    // depending on distance

    todo!()
}

pub fn process_part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_works() {
        let expected = String::from("374");
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

    #[test]
    fn isolate_column_works() {
        let input: Vec<Vec<Tile>> = vec![
            vec![Tile::Space, Tile::Galaxy, Tile::Galaxy],
            vec![Tile::Galaxy, Tile::Space, Tile::Space],
            vec![Tile::Galaxy, Tile::Galaxy, Tile::Space],
        ];
        let expected = vec![Tile::Space, Tile::Galaxy, Tile::Galaxy];
        let result: Vec<Tile> = isolate_column(&input, 0_usize).collect();
        assert_eq!(expected, result);
    }

    #[test]
    fn galacticpair_partialeq_works() {
        let this = GalacticPair(Pos { x: 1, y: 2 }, Pos { x: 2, y: 1 });
        let that = GalacticPair(Pos { x: 1, y: 2 }, Pos { x: 2, y: 1 });
        let other = GalacticPair(Pos { x: 2, y: 1 }, Pos { x: 1, y: 2 });
        let bad = GalacticPair(Pos { x: 3, y: 4 }, Pos { x: 4, y: 3 });
        assert_eq!(this, that);
        assert_eq!(this, other);
        assert_ne!(this, bad);
    }
}
