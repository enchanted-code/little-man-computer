use std::io::{BufRead, Write};

use crate::assembler;

pub trait Runtime<'a> {
    /// Load from a assembled program
    fn load_assembled(memory: &'a mut [usize; 100]) -> Self;
    /// Run whole program until completion.
    fn run(&mut self) {
        while !self.step() {}
    }
    /// Step next instruction in program,
    /// returning whether the program is complete.
    fn step(&mut self) -> bool;
}

pub struct CommandLine<'a> {
    memory: &'a mut [usize; 100],
    program_counter: usize,
    accumulator: usize,
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
}

impl<'a> Runtime<'a> for CommandLine<'a> {
    fn load_assembled(memory: &'a mut [usize; 100]) -> Self {
        Self {
            memory,
            program_counter: 0,
            accumulator: 0,
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }
    fn step(&mut self) -> bool {
        let opcode = assembler::extract_opcode_from_assembled(self.memory[self.program_counter]);
        let value = assembler::extract_value_from_assembled(self.memory[self.program_counter]);
        match (opcode, value) {
            (assembler::OPCODE_ADD, _) => {
                let result = (self.memory[value] + self.accumulator).clamp(0, 999);
                self.accumulator = result;
            }
            (assembler::OPCODE_SUB, _) => {
                let result = (self.memory[value] - self.accumulator).clamp(0, 999);
                self.accumulator = result;
            }
            (assembler::OPCODE_STA, _) => self.memory[value] = self.accumulator,
            (assembler::OPCODE_LDA, _) => self.accumulator = self.memory[value],
            (assembler::OPCODE_BRA, _) => {
                self.program_counter = value;
                return false;
            }
            (assembler::OPCODE_BRZ, _) => {
                if self.accumulator == 0 {
                    self.program_counter = value;
                    return false;
                }
            }
            (assembler::OPCODE_BRP, _) => {
                if self.accumulator > 0 {
                    self.program_counter = value;
                    return false;
                }
            }
            (assembler::OPCODE_INP, 1) => {
                let mut handle = self.stdout.lock();
                handle.write_all(format!("<<< ").as_bytes()).unwrap();
                self.stdout.flush().unwrap();
                let mut handle = self.stdin.lock();
                let mut buf = String::new();
                handle.read_line(&mut buf).unwrap();
                let buf = buf.strip_suffix("\n").unwrap().trim();
                let value = buf.parse().unwrap();
                self.accumulator = value;
            }
            (assembler::OPCODE_OUT, 2) => {
                let mut handle = self.stdout.lock();
                handle
                    .write_all(format!(">>> {}\n", self.accumulator.to_string()).as_bytes())
                    .unwrap();
                self.stdout.flush().unwrap();
            }
            (assembler::OPCODE_HLT, _) => return true,
            _ => unreachable!(),
        }
        if self.program_counter == self.memory.len() {
            return true;
        }
        self.program_counter += 1;
        false
    }
}
