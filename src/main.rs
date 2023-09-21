use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum ListElement {
    Num(i32),
    List(Vec<ListElement>),
}

type LE = ListElement;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn str_to_list(string: &str) -> Vec<LE> {
    let mut output: Vec<LE> = vec![];
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

fn parse_input() -> Vec<(Vec<LE>, Vec<LE>)> {
    let mut output: Vec<(Vec<LE>, Vec<LE>)> = vec![];
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

fn compare(left: &[LE], right: &[LE]) -> Ordering {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    while let (Some(left_option), Some(right_option)) = (left_iter.next(), right_iter.next()) {
        match (left_option, right_option) {
            (LE::Num(left_item), LE::Num(right_item)) => {
                let cmp_result = left_item.cmp(right_item);
                if cmp_result != Ordering::Equal {
                    return cmp_result;
                }
            }
            (LE::List(left_item), LE::List(right_item)) => {
                let cmp_result = compare(left_item, right_item);
                if cmp_result != Ordering::Equal {
                    return cmp_result;
                }
            }
            (LE::List(left_item), LE::Num(right_item)) => {
                let cmp_result = compare(left_item, &[LE::Num(*right_item)]);
                return match cmp_result {
                    Ordering::Less => Ordering::Less,
                    _ => Ordering::Greater,
                };
            }
            (LE::Num(left_item), LE::List(right_item)) => {
                let cmp_result = compare(&[LE::Num(*left_item)], right_item);
                return match cmp_result {
                    Ordering::Greater => Ordering::Greater,
                    _ => Ordering::Less,
                };
            }
        }
    }

    Ordering::Equal
}

fn run_logic(pairs: Vec<(Vec<LE>, Vec<LE>)>) -> Vec<usize> {
    let mut output: Vec<usize> = vec![];

    for (index, (left, right)) in pairs.iter().enumerate() {
        if let Ordering::Less = compare(left, right) {
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
