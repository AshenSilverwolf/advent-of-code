use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use itertools::{Itertools, iproduct};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
// Note: HashSet requires that if k1 == k2, then hash(k1) == hash(k2).
// this could be difficult to implement with our GalacticPair equality logic
// Another Note: BTreeSet requires Ord be implemented for its elements
// this is also very difficult to implement with GalacticPair
// consider just using a Vec? we just need to ensure all pairs are unique

#[derive(Debug, Clone)]
struct GalacticPair(Pos, Pos);

impl From<(Pos, Pos)> for GalacticPair {
    fn from(pair: (Pos, Pos)) -> Self {
        Self(pair.0, pair.1)
    }
}

// TODO: implement Hash for GalacticPair
// hash each member, then combine using ^
// https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html
impl Hash for GalacticPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state) ^ self.1.hash(state)
    }
}

impl PartialEq for GalacticPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

impl Eq for GalacticPair {}

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

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
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

fn isolate_column_as_iter(galactic_map: &Vec<Vec<Tile>>, col_index: usize) -> impl Iterator<Item = Tile> {
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
    let mut galaxy_pairs: Vec<GalacticPair> = vec![];
    let galx_1 = galaxies.clone();
    let galx_2 = galaxies.clone();
    let galaxy_pairs: Vec<GalacticPair> = iproduct!(galx_1.iter(), galx_2.iter())
        .map(|(g1, g2)| GalacticPair(g1.clone(), g2.clone()))
        // .map(GalacticPair::from) // map to GalacticPair type
        .filter(|GalacticPair(f, s)| f != s) // remove pairs where both are the same
        .unique() // remove references to the same pair, regardless of order
        .collect::<Vec<GalacticPair>>(); // collect into Vec
    dbg!(&galaxy_pairs);
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
        let result: Vec<Tile> = isolate_column_as_iter(&input, 0_usize).collect();
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

    #[test]
    fn pair_generation_works() {
        let expected: Vec<GalacticPair> = vec![
            GalacticPair(Pos::new(1, 2), Pos::new(3, 6)),
            GalacticPair(Pos::new(1, 2), Pos::new(4, 8)),
            GalacticPair(Pos::new(3, 6), Pos::new(4, 8)),
        ];
        let galaxies: Vec<Pos> = vec![
            Pos::new(1, 2),
            Pos::new(3, 6),
            Pos::new(4, 8),
        ];
        let g1 = galaxies.clone();
        let g2 = galaxies.clone();
        let galaxy_pairs: Vec<GalacticPair> = iproduct!(g1.iter(), g2.iter())
            .map(|(g1, g2)| GalacticPair(g1.clone(), g2.clone()))
            .filter(|GalacticPair(f, s)| f != s)
            .unique()
            .collect::<Vec<GalacticPair>>();
        assert_eq!(expected, galaxy_pairs);
    }
}
