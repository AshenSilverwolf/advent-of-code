use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct SysDir {
    name: String,
    size: usize,
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
        name: String::from("/"),
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
                    name: path.join("/") + "/" + tokens[1],
                    size: 0,
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
    let mut sum: usize = 0;
    for dir in directories.iter().filter(|x| x.size <= 100000) {
        sum += dir.size;
    }
    println!("{}", sum);
}
