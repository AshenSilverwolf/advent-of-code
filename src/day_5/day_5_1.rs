use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NUM_COLS: i32 = 9;

enum State {
    Stacks,
    Commands,
}

#[derive(Debug, Clone)]
struct Stack {
    id: i32,
    stack: Vec<char>,
}

#[derive(Debug, Copy, Clone)]
struct Command {
    num_items: i32,
    from: i32,
    to: i32,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> (Vec<Stack>, Vec<Command>) {
    let mut state: State = State::Stacks;
    let mut stacks: Vec<Stack> = vec![];
    let mut reverse_stacks: Vec<Stack> = vec![];
    let mut commands: Vec<Command> = vec![];

    for i in 0..NUM_COLS {
        reverse_stacks.push(Stack {
            id: i,
            stack: vec![],
        });
    }

    if let Ok(lines) = read_lines("./test.txt") {
        for line in lines.into_iter().flatten() {
            if line.is_empty() {
                state = State::Commands;
                continue;
            }

            match state {
                State::Stacks => {
                    let chars: Vec<char> = line.chars().collect();
                    for i in 0..NUM_COLS as usize {
                        let c: char = chars[(i * 4) + 1];
                        if c.is_alphabetic() {
                            reverse_stacks[i].stack.push(c)
                        } else if c.is_numeric() {
                            break;
                        }
                    }
                }
                State::Commands => {
                    // implement parsing of commands
                    let words: Vec<i32> = line
                        .split_whitespace()
                        .flat_map(|x| -> Result<i32, <i32 as std::str::FromStr>::Err> { x.parse() })
                        .collect();
                    // println!("words len: {}", words.len());
                    let curr_command = Command {
                        num_items: words[0],
                        from: words[1] - 1,
                        to: words[2] - 1,
                    };
                    commands.push(curr_command);
                }
            }
        }
    }

    for stack in reverse_stacks {
        stacks.push(Stack {
            id: stack.id,
            stack: stack.stack.iter().cloned().rev().collect(),
        })
    }

    (stacks, commands)
}

fn run_logic(stacks: &mut [Stack], commands: Vec<Command>) -> String {
    let mut output_string = String::from("");

    for command in commands {
        for _ in 0..command.num_items as usize {
            let element: char = stacks[command.from as usize].stack.pop().unwrap();
            stacks[command.to as usize].stack.push(element);
        }
    }

    for stack in stacks {
        let c: char = stack.stack.pop().unwrap();
        output_string.push(c);
    }

    output_string
}

fn main() {
    let (mut stacks, commands) = parse_input();
    let final_tops: String = run_logic(&mut stacks, commands);
    println!("{}", final_tops);
}
