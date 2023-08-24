use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> Vec<Vec<u32>> {
    let mut forest: Vec<Vec<u32>> = vec![];

    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.into_iter().flatten() {
            let row: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            forest.push(row);
        }
    }

    forest
}

fn is_visible<'a, T>(cell: &'a u32, slice: T) -> bool
where
    T: IntoIterator,
    <T as IntoIterator>::Item: PartialOrd<&'a u32>,
{
    for x in slice {
        if x >= cell {
            return false;
        }
    }

    return true;
}

fn run_logic(forest: Vec<Vec<u32>>) -> usize {
    let (width, height) = (forest[0].len(), forest.len());
    let mut num_visible = (width * 2) + (height * 2) - 4;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let curr = &forest[i][j];
            let north: Vec<u32> = forest[..i].iter().map(|x| x[j]).collect();
            let south: Vec<u32> = forest[i + 1..].iter().map(|x| x[j]).collect();
            let east = &forest[i][j + 1..];
            let west = &forest[i][..j];

            let vis_e = is_visible(curr, east);
            let vis_s = is_visible(curr, &south);
            let vis_w = is_visible(curr, west);
            let vis_n = is_visible(curr, &north);

            if vis_e || vis_s || vis_w || vis_n {
                num_visible += 1;
            }
        }
    }

    num_visible
}

fn main() {
    let forest = parse_input();
    let num_visible = run_logic(forest);
    println!("{}", num_visible);
}
