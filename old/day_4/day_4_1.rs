use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn separate_ranges(text: String) -> ((i32, i32), (i32, i32)) {
    let split: Vec<&str> = text.split(',').collect();
    let elf_1_split = split[0];
    let elf_2_split = split[1];
    let elf_1_num_split: Vec<&str> = elf_1_split.split('-').collect();
    let elf_2_num_split: Vec<&str> = elf_2_split.split('-').collect();
    let elf_1_lo: i32 = elf_1_num_split[0].parse().unwrap();
    let elf_1_hi: i32 = elf_1_num_split[1].parse().unwrap();
    let elf_1: (i32, i32) = (elf_1_lo, elf_1_hi);
    let elf_2_lo: i32 = elf_2_num_split[0].parse().unwrap();
    let elf_2_hi: i32 = elf_2_num_split[1].parse().unwrap();
    let elf_2: (i32, i32) = (elf_2_lo, elf_2_hi);

    (elf_1, elf_2)
}

fn parse_input() -> Vec<((i32, i32), (i32, i32))> {
    let mut range_pairs: Vec<((i32, i32), (i32, i32))> = vec![];
    if let Ok(lines) = read_lines("./test.txt") {
        for line in lines {
            if let Ok(text) = line {
                let elf_ranges: ((i32, i32), (i32, i32)) = separate_ranges(text);
                range_pairs.push(elf_ranges);
            }
        }
    }

    range_pairs
}

fn run_logic(range_pairs: Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut subset_counter = 0;
    for (range1, range2) in range_pairs {
        let first: HashSet<i32> = (range1.0..=range1.1).into_iter().collect();
        let second: HashSet<i32> = (range2.0..=range2.1).into_iter().collect();
        if first.is_superset(&second) || second.is_superset(&first) {
            subset_counter += 1;
        }
    }

    subset_counter
}

fn main() {
    let range_pairs: Vec<((i32, i32), (i32, i32))> = parse_input();
    let num_overlapping_ranges: i32 = run_logic(range_pairs);
    println!("{}", num_overlapping_ranges);
}