use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct SysDir {
    name: String,
    contents: Vec<SysElm>,
}

#[derive(Debug)]
struct SysFile {
    name: String,
    size: i32,
}

#[derive(Debug)]
enum SysElm {
    SysDir(SysDir),
    SysFile(SysFile),
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() {
    let mut root = SysDir {
        name: String::from("/"),
        contents: vec![],
    };
    let mut cwd = &mut root;
    cwd.contents.push(SysElm::SysFile(SysFile {
        name: String::from("testFile"),
        size: 29116,
    }));
    cwd.contents.push(SysElm::SysDir(SysDir {
        name: String::from("a"),
        contents: vec![],
    }));
    println!("{:#?}", &cwd);
    if let SysElm::SysDir(nwd) = &mut cwd.contents[1] {
        cwd = nwd;
    }
    cwd.contents.push(SysElm::SysFile(SysFile {
        name: String::from("b"),
        size: 12345,
    }));

    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.into_iter().flatten() {
            let words: Vec<&str> = line.split_whitespace().collect();
            if words[0] == "$" {
                todo!();
            } else {
                todo!();
            }
        }
    }
}

fn main() {
    parse_input();
}
