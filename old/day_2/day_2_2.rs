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

fn determine_p2_shape(elf: &Shape, outcome: &Outcome) -> Shape {
	match (elf, outcome) {
		(Shape::Rock, Outcome::Win) => Shape::Paper,
		(Shape::Rock, Outcome::Loss) => Shape::Scissors,
		(Shape::Rock, Outcome::Draw) => Shape::Rock,
		(Shape::Paper, Outcome::Win) => Shape::Scissors,
		(Shape::Paper, Outcome::Loss) => Shape::Rock,
		(Shape::Paper, Outcome::Draw) => Shape::Paper,
		(Shape::Scissors, Outcome::Win) => Shape::Rock,
		(Shape::Scissors, Outcome::Loss) => Shape::Paper,
		(Shape::Scissors, Outcome::Draw) => Shape::Scissors,
	}
}

fn get_shape_from_char(c: &str) -> Option<Shape> {
	match c {
		"A" => Some(Shape::Rock),
		"B" => Some(Shape::Paper),
		"C" => Some(Shape::Scissors),
		_ => None,
	}
}

fn get_outcome_from_char(c: &str) -> Option<Outcome> {
	match c {
		"X" => Some(Outcome::Loss),
		"Y" => Some(Outcome::Draw),
		"Z" => Some(Outcome::Win),
		_ => None,
	}
}

fn play_games(games: Vec<(String, String)>) -> Option<i32> {
	let mut running_score: i32 = 0;

    for game in games {
		let p1: Shape = get_shape_from_char(&game.0)?;
		let game_outcome: Outcome = get_outcome_from_char(&game.1)?;
        let p2: Shape = determine_p2_shape(&p1, &game_outcome);
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
