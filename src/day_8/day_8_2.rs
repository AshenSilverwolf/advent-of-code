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

fn calc_directional_scenic_score(cell: &u32, slice: &Vec<u32>) -> u32 {
    let mut score: u32 = 0;
    for x in slice {
        score += 1;
        if x >= cell {
            break;
        }
    }

    score
}

fn run_logic(forest: Vec<Vec<u32>>) -> u32 {
    let (width, height) = (forest[0].len(), forest.len());
    let mut high_score: u32 = 0;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let curr = &forest[i][j];
            let mut north: Vec<u32> = forest[..i].iter().map(|x| x[j]).collect();
            let south: Vec<u32> = forest[i + 1..].iter().map(|x| x[j]).collect();
            let east: Vec<u32> = forest[i][j + 1..].to_vec();
            let mut west: Vec<u32> = forest[i][..j].to_vec();

            north.reverse();
            west.reverse();

            let score_e = calc_directional_scenic_score(curr, &east);
            let score_s = calc_directional_scenic_score(curr, &south);
            let score_w = calc_directional_scenic_score(curr, &west);
            let score_n = calc_directional_scenic_score(curr, &north);

            let cell_score = score_e * score_s * score_w * score_n;

            high_score = if cell_score > high_score {
                cell_score
            } else {
                high_score
            };
        }
    }

    high_score
}

fn main() {
    let forest = parse_input();
    let scenic_score = run_logic(forest);
    println!("{}", scenic_score);
}
