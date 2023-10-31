use day_15::process_part1;
use std::fs;

const TARGET_ROW: i32 = 2_000_000;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file, TARGET_ROW));
}
