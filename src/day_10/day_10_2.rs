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

fn render(mut display: Vec<Vec<String>>, register: &i32, cycle: &i32) -> Vec<Vec<String>> {
    let mut temp = *cycle;
    let mut row: usize = 0;
    while temp > 40 {
        temp -= 40;
        row += 1;
    }
    let col: usize = (temp as usize - 1) % 40;

    let sprite: Vec<i32> = vec![register - 1, *register, register + 1];
    let pixel: i32 = (cycle - 1) % 40;

    if sprite.contains(&pixel) {
        display[row][col] = "#".to_string();
    }

    display
}

fn run_logic(instructions: Vec<Instruction>) -> Vec<Vec<String>> {
    let num_instr: usize = instructions.len();
    let mut instr_counter: usize = 0;
    let mut register: i32 = 1;
    let mut ticks: i32 = 1;
    let mut state: LoopState = LoopState::Run;
    let mut crt_display: Vec<Vec<String>> = Vec::with_capacity(6);
    for i in 0..6 {
        crt_display.push(Vec::with_capacity(40));
        for _ in 0..40 {
            crt_display[i].push(".".to_string());
        }
    }

    while instr_counter < num_instr {
        crt_display = render(crt_display, &register, &ticks);

        let instr = &instructions[instr_counter];
        if let Instruction::Add(x) = instr {
            match state {
                LoopState::Wait(y) => {
                    register += y;
                    state = LoopState::Run;
                    instr_counter += 1;
                }
                LoopState::Run => {
                    state = LoopState::Wait(*x);
                }
            };
        } else {
            instr_counter += 1;
        }

        ticks += 1
    }

    crt_display
}

fn display(crt_screen: Vec<Vec<String>>) {
    for row in crt_screen {
        println!("{}", row.join(""));
    }
}

fn main() {
    let instructions = parse_input();
    let crt_screen = run_logic(instructions);
    display(crt_screen);
}
