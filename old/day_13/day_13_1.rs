use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListElement {
    Num(i32),
    List(Vec<ListElement>),
}

impl PartialOrd for ListElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (LE::Num(left), LE::Num(right)) => left.partial_cmp(right),
            (LE::Num(left), LE::List(right)) => {
                [LE::List(vec![LE::Num(*left)])].partial_cmp(&[LE::List(right.to_vec())])
            }
            (LE::List(left), LE::Num(right)) => {
                [LE::List(left.to_vec())].partial_cmp(&[LE::List(vec![LE::Num(*right)])])
            }
            (LE::List(left), LE::List(right)) => {
                for (l, r) in left.iter().zip(right) {
                    match l.partial_cmp(r) {
                        Some(Ordering::Equal) => (),
                        None => (),
                        ordering => return ordering,
                    }
                }

                left.len().partial_cmp(&right.len())
            }
        }
    }
}

impl Ord for ListElement {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ordering) => ordering,
            None => unreachable!(),
        }
    }
}

type LE = ListElement;
type Packet = Vec<LE>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn str_to_list(string: &str) -> Packet {
    let mut output: Packet = vec![];
    let mut chars = string.chars();
    let mut index = 0;

    while let Some(c) = chars.next() {
        match c {
            '[' => {
                let start = index + 1;
                let mut stack: Vec<char> = vec!['['];
                while !stack.is_empty() {
                    let c2 = chars.next();
                    match c2 {
                        Some('[') => {
                            stack.push('[');
                        }
                        Some(']') => {
                            stack.pop();
                        }
                        Some(_) => {}
                        None => {
                            panic!();
                        }
                    };

                    index += 1;
                }

                output.push(LE::List(str_to_list(&string[start..index])));
            }
            ']' => {
                panic!();
            }
            ',' => {}
            _ => {
                if !c.is_numeric() {
                    panic!();
                }

                let mut num_str = String::from(c);
                if let Some(n2) = chars.next() {
                    if n2.is_numeric() {
                        num_str.push(n2);
                    }
                }

                let num = num_str.parse::<i32>().unwrap();

                index += 1;
                output.push(LE::Num(num));
            }
        }

        index += 1;
    }

    output
}

fn parse_input() -> Vec<(Packet, Packet)> {
    let mut output: Vec<(Packet, Packet)> = vec![];
    if let Ok(lines) = read_lines("test.txt") {
        let mut lines_iter = lines.into_iter().flatten();
        loop {
            let left_str = lines_iter.next().unwrap();
            let left_len = left_str.len();
            let right_str = lines_iter.next().unwrap();
            let right_len = right_str.len();
            let left = str_to_list(&left_str[1..left_len - 1]);
            let right = str_to_list(&right_str[1..right_len - 1]);

            output.push((left, right));

            if lines_iter.next().is_none() {
                break;
            }
        }
    }

    output
}

fn run_logic(pairs: Vec<(Packet, Packet)>) -> Vec<usize> {
    let mut output: Vec<usize> = vec![];

    for (index, (left, right)) in pairs.iter().enumerate() {
        if let Ordering::Less = left.cmp(right) {
            output.push(index + 1);
        }
    }

    output
}

fn main() {
    let pairs_list = parse_input();
    let ordered_indices = run_logic(pairs_list);
    let sum: usize = ordered_indices.iter().sum();
    println!("{}", sum);
}
