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

fn parse_input() -> Vec<i32> {
    let mut elves_cals = vec![];
    if let Ok(lines) = read_lines("day1_input.txt") {
        let mut curr_cals = 0;
        for line in lines {
            let cals = line.unwrap();
            if cals.len() == 0 {
                elves_cals.push(curr_cals);
                curr_cals = 0;
                continue;
            }
            curr_cals += cals.parse::<i32>().unwrap();
        }
    }

    elves_cals
}

fn get_max_cals(elves: Vec<i32>) -> i32 {
    let mut max = 0;
    for cals in elves {
        if cals > max {
            max = cals;
        }
    }

    max
}

fn main() {
    let elves_cals = parse_input();
    let max_cals = get_max_cals(elves_cals);
    println!("{}", max_cals);
}
