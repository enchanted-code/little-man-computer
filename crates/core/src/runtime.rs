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

impl CommandLine<'_> {
    fn write_stdout(&mut self, content: &str) {
        let mut handle = self.stdout.lock();
        handle.write_all(content.as_bytes()).unwrap();
        self.stdout.flush().unwrap();
    }

    fn read_stdin(&mut self, buf: &mut String) {
        buf.clear();
        let mut handle = self.stdin.lock();
        handle.read_line(buf).unwrap();
        *buf = buf.strip_suffix("\n").unwrap().trim().to_string();
    }
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
                let mut ok = false;
                while !ok {
                    self.write_stdout(&format!("<<< "));
                    let mut buf = String::new();
                    self.read_stdin(&mut buf);
                    if let Ok(value) = buf.parse::<usize>() {
                        self.accumulator = value.min(999);
                        ok = true;
                    } else {
                        self.write_stdout("not a valid number\n");
                        ok = false;
                    }
                }
            }
            (assembler::OPCODE_OUT, 2) => {
                self.write_stdout(&format!(">>> {}\n", self.accumulator.to_string()));
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
