use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> Vec<i32> {
	let mut elves_cals: Vec<i32> = vec!();
    if let Ok(lines) = read_lines("day1_input.txt") {
		let mut curr_cals: i32 = 0;
        for line in lines {
			let cals: String = line.unwrap();
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

fn get_three_most_cals(mut elves: Vec<i32>) -> Vec<i32> {
	elves.sort();
	let max1: i32 = elves.pop().unwrap();
	let max2: i32 = elves.pop().unwrap();
	let max3: i32 = elves.pop().unwrap();

	vec!(max1, max2, max3)
}

fn main() {
    let elves_cals: Vec<i32> = parse_input();
	let top_three: Vec<i32> = get_three_most_cals(elves_cals);
	let total: i32 = top_three.iter().sum();
	println!("{:?}", total);
}