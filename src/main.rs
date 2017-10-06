use std::io::{stdin, stdout, Write};
use std::io::Read;
use std::collections::HashMap;

mod types;
mod errors;

use types::BrainCommand;
use errors::BrainError;


/// Size of the memory tape
const TAPE_SIZE: usize = 30_000;
static VALID_COMMANDS: &[char] = &['>', '<', '+', '-', '.', ',', '[', ']'];

fn validate(program: &Vec<BrainCommand>) -> bool {
    for command in program {
        match *command {
            BrainCommand::JumpBackward(pos) | BrainCommand::JumpForward(pos) => if let None = pos {
                return false;
            },
            _ => continue,
        }
    }
    true
}

fn compile(source: &str) -> Result<Vec<BrainCommand>, BrainError> {
    let str_commands: Vec<_> = source
        .chars()
        .filter(|c| VALID_COMMANDS.contains(c))
        .collect();
    if str_commands.len() == 0 {
        return Err(BrainError::ParseError);
    }

    use BrainCommand::*;

    let mut program = Vec::new();
    let mut stack = Vec::new();
    let mut pc = 0;

    for command in &str_commands {
        let operation = match *command {
            '>' => MoveRight,
            '<' => MoveLeft,
            '+' => Increment,
            '-' => Decrement,
            '.' => Output,
            ',' => Input,
            '[' => {
                stack.push(pc);
                JumpForward(None)
            }
            ']' => if let Some(jmp_pc) = stack.pop() {
                if let JumpForward(ref mut pos) = program[jmp_pc] {
                    *pos = Some(jmp_pc);
                };
                JumpBackward(Some(jmp_pc))
            } else {
                return Err(BrainError::ParseError);
            },
            _ => return Err(BrainError::ParseError),
        };
        program.push(operation);
        pc += 1;
    }
    if validate(&program) {
        Ok(program)
    } else {
        Err(BrainError::ParseError)
    }
}

fn interpret(program: &Vec<BrainCommand>) {
    use BrainCommand::*;
    let mut tape = [0i32; TAPE_SIZE];
    let mut pointer = 0;
    let mut pc = 0;

    loop {
        if pc >= program.len() {
            break;
        }
        match program[pc] {
            Decrement => tape[pointer] = tape[pointer].wrapping_sub(1),
            Increment => tape[pointer] = tape[pointer].wrapping_add(1),
            Input => {
                tape[pointer] = stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as i32)
                    .unwrap()
            }
            JumpBackward(pos) => if tape[pointer] != 0 {
                pc = pos.unwrap();
            },
            JumpForward(pos) => if tape[pointer] == 0 {
                pc = pos.unwrap();
            },
            MoveLeft => pointer -= 1,
            MoveRight => pointer += 1,
            Output => {
                print!("{}", tape[pointer] as u8 as char);
            }
        }
        pc += 1;
    }
}

fn main() {
    let mut x = String::new();
    stdin().read_line(&mut x).unwrap();
    let src = x.trim().to_string();
    if let Ok(ast) = compile(&src) {
        interpret(&ast);
    } else {
        eprintln!("Invalid brainf code");
    }
}
