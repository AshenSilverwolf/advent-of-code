use nom::{
    IResult,
    multi::{separated_list1, many1},
    combinator::map,
    character::complete::{newline, one_of},
};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Space,
    Galaxy,
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(
        newline,
        many1(map(
            one_of(".#"),
            |c: char| match c {
                '.' => Tile::Space,
                '#' => Tile::Galaxy,
                _ => panic!("invalid input"),
            },
        )),
    )(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, galactic_map) = parse_input(input).expect("valid output");
    let num_galaxies: usize = galactic_map
        .iter()
        .map(|row| row.iter().filter(|tile| tile == &&Tile::Galaxy).count())
        .sum();

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
}
