use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use itertools::{Itertools, iproduct};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct GalacticPair(Pos, Pos);

impl From<(Pos, Pos)> for GalacticPair {
    fn from(pair: (Pos, Pos)) -> Self {
        Self(pair.0, pair.1)
    }
}

impl Hash for GalacticPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0.id() ^ self.1.id()).hash(state);
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

    fn id(&self) -> usize {
        (self.x * 0x1f1f1f1f) ^ self.y
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Alignment {
    Vert,
    Horiz,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EmptySpace(Alignment, usize);

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
    if col_index >= galactic_map[0].len() {
        panic!("col_index exceeds max size of map: {col_index}");
    }
    let col: Vec<Tile> = galactic_map
        .iter()
        .map(|row| row[col_index].clone())
        .collect();
    col.into_iter()
}

fn generate_galaxy_set(galaxy_map: &Vec<Vec<Tile>>) -> HashSet<Pos> {
    galaxy_map
        .iter()
        .enumerate()
        .flat_map(|(y_index, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, t)| t == &&Tile::Galaxy)
                .map(|(x_index, _)| Pos::new(x_index, y_index))
                .collect::<Vec<Pos>>()
        })
        .collect()
}

fn generate_galaxy_pairs(galaxies: &HashSet<Pos>) -> Vec<GalacticPair> {
    let galx_1 = galaxies.clone();
    let galx_2 = galaxies.clone();
    iproduct!(galx_1.iter(), galx_2.iter())
        .map(|(g1, g2)| GalacticPair(g1.clone(), g2.clone()))
        .filter(|GalacticPair(f, s)| f != s)
        .unique()
        .collect()
}

fn collect_empty_space(galaxy_map: &Vec<Vec<Tile>>) -> Vec<EmptySpace> {
    let mut empty_space: Vec<EmptySpace> = vec![];
    for i in 0..galaxy_map.len() {
        let row = galaxy_map[i].clone();
        if row.iter().any(|t| t == &Tile::Galaxy) {
            continue;
        }
        empty_space.push(EmptySpace(Alignment::Horiz, i));
    }
    for j in 0..galaxy_map[0].len() {
        let mut col = isolate_column_as_iter(&galaxy_map, j);
        if col.any(|t| t == Tile::Galaxy) {
            continue;
        }
        empty_space.push(EmptySpace(Alignment::Vert, j));
    }

    empty_space
}

pub fn process_part1(input: &str) -> String {
    let (_, galaxy_map) = parse_input(input).expect("valid input");

    // create set of all unique pairs of galaxies
    let galaxies: HashSet<Pos> = generate_galaxy_set(&galaxy_map);
    let galaxy_pairs: Vec<GalacticPair> = generate_galaxy_pairs(&galaxies);
    // identify and store all blank lines in space
    let empty_space = collect_empty_space(&galaxy_map);
    // find distance between all pairs
    galaxy_pairs
        .into_iter()
        .map(|GalacticPair(this, that)| (this.distance(&that), GalacticPair(this, that)))
        .map(|(dist, GalacticPair(this, that))| {
            let mut expanded_dist = dist;
            expanded_dist += empty_space.iter().filter(|&EmptySpace(alignment, row)|
                *alignment == Alignment::Horiz 
                && (
                    this.y < *row
                    && *row < that.y
                    || that.y < *row
                    && *row < this.y
                )
            )
            .count();
            expanded_dist += empty_space.iter().filter(|&EmptySpace(alignment, col)|
                *alignment == Alignment::Vert
                && (
                    this.x < *col
                    && *col < that.x
                    ||
                    that.x < *col
                    && *col < this.x
                )
            )
            .count();

            expanded_dist
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, galaxy_map) = parse_input(input).expect("valid input");
    let galaxies: HashSet<Pos> = generate_galaxy_set(&galaxy_map);
    let galaxy_pairs: Vec<GalacticPair> = generate_galaxy_pairs(&galaxies);
    let empty_space = collect_empty_space(&galaxy_map);
    galaxy_pairs
        .into_iter()
        .map(|GalacticPair(this, that)| (this.distance(&that), GalacticPair(this, that)))
        .map(|(dist, GalacticPair(this, that))| {
            let mut expanded_dist = dist;
            expanded_dist += empty_space.iter().filter(|&EmptySpace(alignment, row)|
                *alignment == Alignment::Horiz 
                && (
                    this.y < *row
                    && *row < that.y
                    || that.y < *row
                    && *row < this.y
                )
            )
            .count() * 10;
            expanded_dist += empty_space.iter().filter(|&EmptySpace(alignment, col)|
                *alignment == Alignment::Vert
                && (
                    this.x < *col
                    && *col < that.x
                    ||
                    that.x < *col
                    && *col < this.x
                )
            )
            .count() * 10;

            expanded_dist
        })
        .sum::<usize>()
        .to_string()
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
    fn part2_works() {
        let expected_10 = String::from("1030");
        let expected_100 = String::from("8410");
        let result = process_part2(INPUT);
        assert_eq!(expected_10, result);
        // assert_eq!(expected_100, result);
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

    #[test]
    fn empty_space_generation_works() {
        let (_, galaxy_map) = parse_input(INPUT).expect("valid input");
        let expected: Vec<EmptySpace> = vec![
            EmptySpace(Alignment::Horiz, 3),
            EmptySpace(Alignment::Horiz, 7),
            EmptySpace(Alignment::Vert, 2),
            EmptySpace(Alignment::Vert, 5),
            EmptySpace(Alignment::Vert, 8),
        ];
        let result = collect_empty_space(&galaxy_map);
        assert_eq!(expected, result);
    }
}
