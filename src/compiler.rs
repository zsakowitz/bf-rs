//! A compiler for Brainf*** programs.

use crate::runner::Runner;

#[derive(Clone, Debug)]
enum Instruction {
    Increment,
    Decrement,
    ShiftLeft,
    ShiftRight,
    Read,
    Write,
    Loop(Vec<Instruction>),
}

fn parse(source: &str) -> Result<Program, &'static str> {
    let mut all_lists: Vec<Vec<Instruction>> = Vec::new();
    let mut current_list: Vec<Instruction> = Vec::new();

    for char in source.chars() {
        match char {
            '+' => current_list.push(Instruction::Increment),
            '-' => current_list.push(Instruction::Decrement),
            '<' => current_list.push(Instruction::ShiftLeft),
            '>' => current_list.push(Instruction::ShiftRight),
            ',' => current_list.push(Instruction::Read),
            '.' => current_list.push(Instruction::Write),

            '[' => {
                let sub_instruction_list: Vec<Instruction> = Vec::new();
                all_lists.push(current_list);
                current_list = sub_instruction_list;
            }

            ']' => {
                let sub_instruction_list = current_list;

                let Some(mut last_instruction_list) = all_lists.pop() else {
                    return Err("unmatched closing bracket");
                };

                last_instruction_list.push(Instruction::Loop(sub_instruction_list));
                current_list = last_instruction_list;
            }

            _ => {}
        };
    }

    if !all_lists.is_empty() {
        Err("unmatched opening bracket")
    } else {
        Ok(Program {
            instructions: current_list,
        })
    }
}

#[derive(Clone, Debug)]
/// A parsed program.
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    /// Attempts to parse a program source, returning an error if it is malformed.
    pub fn new(source: &str) -> Result<Self, &'static str> {
        parse(source)
    }

    /// Runs the program on a given input, outputting a `Runner` once complete.
    pub fn run<const N: usize>(&self, input: &[u8]) -> Runner<N> {
        fn run<const N: usize>(runner: &mut Runner<N>, list: &Vec<Instruction>) {
            for instruction in list {
                match instruction {
                    Instruction::Increment => runner.inc(),
                    Instruction::Decrement => runner.dec(),
                    Instruction::ShiftLeft => runner.shl(),
                    Instruction::ShiftRight => runner.shr(),
                    Instruction::Read => runner.read(),
                    Instruction::Write => runner.write(),
                    Instruction::Loop(list) => runner.repeat(|runner| run(runner, &list)),
                }
            }
        }

        let mut runner = Runner::new(input);

        run(&mut runner, &self.instructions);

        runner
    }
}
