use std::io::stdin;
use std::io::Read;
use std::collections::HashMap;

mod types;
mod errors;

use types::BrainCommand;
use errors::BrainError;

static VALID_COMMANDS: &[char] = &['>', '<', '+', '-', '.', ',', '[', ']'];

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
    let mut program_counter = 0;

    for command in &str_commands {
        let operation = match *command {
            '>' => MoveRight,
            '<' => MoveLeft,
            '+' => Increment,
            '-' => Decrement,
            '.' => Output,
            ',' => Input,
            '[' => {
                stack.push(program_counter);
                JumpForward(None)
            },
            ']' => {
                if let Some(jmp_pc) = stack.pop() {
                    if let JumpForward(ref mut pos) = program[jmp_pc] {
                        *pos = Some(jmp_pc);
                    };
                    JumpBackward(Some(jmp_pc))
                } else {
                    return Err(BrainError::ParseError);
                }
            },
            _ => return Err(BrainError::ParseError),
        };
        program.push(operation);
        program_counter += 1;
    }
    Ok(program)
}

fn real_main(src: &str) {
    println!("{:?}", compile(src))
}


fn main() {
    real_main("++[>++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]");
}

#[allow(dead_code)]
static HELLO_WORLD: &'static str = "++++++++[>++++[>++>+++>+++\
                                    >+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.\
                                    >>.<-.<.+++.------.--------.>>+.>++.";
