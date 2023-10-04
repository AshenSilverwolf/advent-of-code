use std::char;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> Vec<(BTreeSet<char>, BTreeSet<char>, BTreeSet<char>)> {
	let mut trios: Vec<(BTreeSet<char>, BTreeSet<char>, BTreeSet<char>)> = vec![];
	let mut trio: Vec<BTreeSet<char>> = vec![];
	let mut counter: i32 = 0;
    if let Ok(lines) = read_lines("test.txt") {
        for line in lines {
			counter += 1;
			if let Ok(text) = line {
				trio.push(text.chars().collect::<BTreeSet<char>>());
			}
			if counter >= 3 {
				let first = trio.pop().unwrap();
				let second = trio.pop().unwrap();
				let third = trio.pop().unwrap();
				trios.push((first, second, third));
				counter = 0;
			}
        }
    }

	trios
}

fn run_logic(trios: &Vec<(BTreeSet<char>, BTreeSet<char>, BTreeSet<char>)>) -> Option<i32> {
	let mut priority_sum = 0;
	for trio in trios {
		let duplicate = trio.0
			.intersection(&trio.1)
			.cloned()
			.collect::<BTreeSet<char>>()
			.intersection(&trio.2)
			.cloned()
			.collect::<Vec<char>>()
			.pop()?;
		let priority = get_priority_from_char(&duplicate)?;
		priority_sum += priority;
	}

	Some(priority_sum)
}

fn get_priority_from_char(c: &char) -> Option<i32> {
	match c {
		'a' => Some(1),
		'b' => Some(2),
		'c' => Some(3),
		'd' => Some(4),
		'e' => Some(5),
		'f' => Some(6),
		'g' => Some(7),
		'h' => Some(8),
		'i' => Some(9),
		'j' => Some(10),
		'k' => Some(11),
		'l' => Some(12),
		'm' => Some(13),
		'n' => Some(14),
		'o' => Some(15),
		'p' => Some(16),
		'q' => Some(17),
		'r' => Some(18),
		's' => Some(19),
		't' => Some(20),
		'u' => Some(21),
		'v' => Some(22),
		'w' => Some(23),
		'x' => Some(24),
		'y' => Some(25),
		'z' => Some(26),
		'A' => Some(27),
		'B' => Some(28),
		'C' => Some(29),
		'D' => Some(30),
		'E' => Some(31),
		'F' => Some(32),
		'G' => Some(33),
		'H' => Some(34),
		'I' => Some(35),
		'J' => Some(36),
		'K' => Some(37),
		'L' => Some(38),
		'M' => Some(39),
		'N' => Some(40),
		'O' => Some(41),
		'P' => Some(42),
		'Q' => Some(43),
		'R' => Some(44),
		'S' => Some(45),
		'T' => Some(46),
		'U' => Some(47),
		'V' => Some(48),
		'W' => Some(49),
		'X' => Some(50),
		'Y' => Some(51),
		'Z' => Some(52),
		_ => None,
	}
}

fn main() {
	let trios: Vec<(BTreeSet<char>, BTreeSet<char>, BTreeSet<char>)> = parse_input();

	if let Some(priority_sum) = run_logic(&trios) {
		println!("{}", priority_sum);
	}
}