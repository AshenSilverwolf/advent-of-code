use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Instruction {
    Skip,
    Add(i32),
}

enum LoopState {
    Run,
    Wait(i32),
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    if let Ok(lines) = read_lines("test.txt") {
        for line in lines.into_iter().flatten() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let curr_instruction: Instruction = match tokens[0] {
                "noop" => Instruction::Skip,
                "addx" => Instruction::Add(tokens[1].parse().unwrap()),
                _ => todo!(),
            };
            instructions.push(curr_instruction);
        }
    }

    instructions
}

fn run_logic(instructions: Vec<Instruction>) -> i32 {
    let mut signal_sum: i32 = 0;
    let mut register: i32 = 1;
    let mut ticks: i32 = 1;
    let key_signals: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut state: LoopState = LoopState::Run;

    let mut instr_iter = instructions.iter();

    loop {
        if key_signals.contains(&ticks) {
            signal_sum += register * ticks;
        }

        if let LoopState::Wait(x) = state {
            register += x;
            ticks += 1;
            state = LoopState::Run;
            continue;
        }

        if let Some(instr) = instr_iter.next() {
            if let Instruction::Add(x) = instr {
                state = LoopState::Wait(*x);
            }
        } else {
            break;
        }

        ticks += 1
    }

    signal_sum
}

fn main() {
    let instructions = parse_input();
    let signal_sum = run_logic(instructions);
    println!("{}", signal_sum);
}
