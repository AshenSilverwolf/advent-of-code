use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const BUFFER_LEN: usize = 14;

enum State {
    Found,
    NotFound,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> String {
    let mut return_string: String = String::from("");

    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.into_iter().flatten() {
            return_string.push_str(&line);
        }
    }

    return_string
}

fn run_logic(buffer_stream: String) -> Option<i32> {
    for i in (BUFFER_LEN - 1)..buffer_stream.len() {
        let mut state = State::Found;
        let marker: Vec<char> = buffer_stream[(i - (BUFFER_LEN - 1))..=i].chars().collect();
        let mut char_set: BTreeSet<char> = BTreeSet::new();
        for c in marker {
            if char_set.contains(&c) {
                state = State::NotFound;
                break;
            }
            char_set.insert(c);
        }

        match state {
            State::Found => return Some((i + 1) as i32),
            State::NotFound => {}
        };
    }

    None
}

fn main() {
    let buffer_stream: String = parse_input();
    let marker_index = run_logic(buffer_stream).unwrap();
    println!("{}", marker_index);
}
