use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
	fn score(&self) -> i32 {
		match self {
			Self::Rock => 1,
			Self::Paper => 2,
			Self::Scissors => 3,
		}
	}
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
	fn score(&self) -> i32 {
		match self {
			Self::Win => 6,
			Self::Loss => 0,
			Self::Draw => 3,
		}
	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> Vec<(String, String)> {
    let mut games: Vec<(String, String)> = vec![];

    if let Ok(lines) = read_lines("day2_input.txt") {
        for line in lines {
            if let Ok(text) = line {
                let split_text: Vec<&str> = text.split_whitespace().collect();
                let (p1, p2) = match &split_text[..] {
                    &[first, second, ..] => (first, second),
                    _ => unreachable!(),
                };
                games.push((p1.to_owned(), p2.to_owned()));
            }
        }
    }

    games
}

fn determine_outcome(elf: &Shape, player: &Shape) -> Outcome {
	match (elf, player) {
		(Shape::Rock, Shape::Rock) => Outcome::Draw,
		(Shape::Rock, Shape::Paper) => Outcome::Win,
		(Shape::Rock, Shape::Scissors) => Outcome::Loss,
		(Shape::Paper, Shape::Rock) => Outcome::Loss,
		(Shape::Paper, Shape::Paper) => Outcome::Draw,
		(Shape::Paper, Shape::Scissors) => Outcome::Win,
		(Shape::Scissors, Shape::Rock) => Outcome::Win,
		(Shape::Scissors, Shape::Paper) => Outcome::Loss,
		(Shape::Scissors, Shape::Scissors) => Outcome::Draw,
	}
}

fn get_shape_from_char(c: &str) -> Option<Shape> {
	match c {
		"A" | "X" => Some(Shape::Rock),
		"B" | "Y" => Some(Shape::Paper),
		"C" | "Z" => Some(Shape::Scissors),
		_ => None,
	}
}

fn play_games(games: Vec<(String, String)>) -> Option<i32> {
	let mut running_score: i32 = 0;

    for game in games {
		let p1: Shape = get_shape_from_char(&game.0)?;
		let p2: Shape = get_shape_from_char(&game.1)?;
        let game_outcome: Outcome = determine_outcome(&p1, &p2);
		running_score += p2.score() + game_outcome.score();
    }

    Some(running_score)
}

fn main() {
    let games: Vec<(String, String)> = parse_input();
    let score: Option<i32> = play_games(games);
    match score {
		Some(score) => {
			println!("{}", score);
		},
		None => { 
			println!("score calculation failed");
		},
	};
}
