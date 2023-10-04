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

fn parse_input() {
    if let Ok(lines) = read_lines("test.txt") {
        for _line in lines.into_iter().flatten() {
            // code here
        }
    }
}

fn main() {
    parse_input();
}