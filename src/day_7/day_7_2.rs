use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://stackoverflow.com/questions/29884402/how-do-i-implement-ord-for-a-struct
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SysDir {
    size: usize,
    name: String,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> Vec<SysDir> {
    let mut directories: Vec<SysDir> = vec![SysDir {
        name: String::from(""),
        size: 0,
    }];
    let mut path: Vec<String> = vec![];

    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.into_iter().flatten() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens[0] == "$" {
                if tokens[1] == "cd" {
                    match tokens[2] {
                        "/" => {
                            path.clear();
                            path.push(String::from(""));
                        }
                        ".." => {
                            path.pop();
                        }
                        _ => {
                            path.push(String::from(tokens[2]));
                        }
                    }
                }
            } else if tokens[0] == "dir" {
                directories.push(SysDir {
                    size: 0,
                    name: path.join("/") + "/" + tokens[1],
                });
            } else {
                let size = if let Ok(size) = tokens[0].parse::<usize>() {
                    size
                } else {
                    panic!()
                };
                for i in 0..path.len() {
                    let dir_in_path = path[0..=i].join("/");
                    for any_dir in &mut directories {
                        if any_dir.name == dir_in_path {
                            any_dir.size += size;
                        }
                    }
                }
            }
        }
    }

    directories
}

fn main() {
    let directories = parse_input();
    let space_max: usize = 70000000;
    let space_required: usize = 30000000;
    let space_used: usize = directories[0].size;
    let space_remaining: usize = space_max - space_used;
    let dir_to_del = directories
        .iter()
        .filter(|x| x.size >= space_required - space_remaining)
        .min()
        .unwrap();
    println!("{}", dir_to_del.size);
}
