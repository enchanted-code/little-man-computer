use std::collections::HashMap;

use crate::ast::{self, Statement};

pub const OPCODE_ADD: usize = 1;
pub const OPCODE_SUB: usize = 2;
pub const OPCODE_STA: usize = 3;
pub const OPCODE_LDA: usize = 5;
pub const OPCODE_BRA: usize = 6;
pub const OPCODE_BRZ: usize = 7;
pub const OPCODE_BRP: usize = 8;
pub const OPCODE_INP: usize = 9;
pub const OPCODE_OUT: usize = 9;
pub const OPCODE_HLT: usize = 0;

pub const ASSEMBLED_OPCODE_ADD: usize = OPCODE_ADD * 100;
pub const ASSEMBLED_OPCODE_SUB: usize = OPCODE_SUB * 100;
pub const ASSEMBLED_OPCODE_STA: usize = OPCODE_STA * 100;
pub const ASSEMBLED_OPCODE_LDA: usize = OPCODE_LDA * 100;
pub const ASSEMBLED_OPCODE_BRA: usize = OPCODE_BRA * 100;
pub const ASSEMBLED_OPCODE_BRZ: usize = OPCODE_BRZ * 100;
pub const ASSEMBLED_OPCODE_BRP: usize = OPCODE_BRP * 100;
pub const ASSEMBLED_OPCODE_INP: usize = OPCODE_OUT * 100 + 1;
pub const ASSEMBLED_OPCODE_OUT: usize = OPCODE_OUT * 100 + 2;
pub const ASSEMBLED_OPCODE_HLT: usize = OPCODE_HLT * 100;

pub fn extract_opcode_from_assembled(assembled: usize) -> usize {
    assembled / 100
}

pub fn extract_value_from_assembled(assembled: usize) -> usize {
    assembled % 100
}

#[derive(Debug)]
pub enum AssemblerError<'a> {
    TooManyInstructions { expected: usize, actual: usize },
    LabelAlreadyDefined { name: &'a str, index: usize },
    LabelNotDefined(&'a str),
}

fn memory_location_to_addr<'a>(
    labels: &HashMap<&str, u8>,
    memory_location: &'a ast::MemoryLocation,
) -> Result<u8, AssemblerError<'a>> {
    match memory_location {
        ast::MemoryLocation::Address(addr) => Ok(*addr),
        ast::MemoryLocation::Label(label) => labels
            .get(label)
            .map(|v| *v)
            .ok_or(AssemblerError::LabelNotDefined(label)),
    }
}

pub fn assemble_from_ast<'a>(
    ast: &'a [ast::Statement<'a>],
    memory: &mut [usize; 100],
) -> Result<(), AssemblerError<'a>> {
    if ast.len() > memory.len() {
        return Err(AssemblerError::TooManyInstructions {
            expected: memory.len(),
            actual: ast.len(),
        });
    }
    let mut labels = HashMap::new();
    for (addr, stmt) in ast.iter().enumerate() {
        if let Statement::Labeled { label, .. } = stmt {
            if labels.insert(label.label, addr as u8).is_some() {
                return Err(AssemblerError::LabelAlreadyDefined {
                    name: label.label,
                    index: addr,
                });
            }
        }
    }
    for (addr, stmt) in memory.iter_mut().zip(ast.iter()) {
        let instruction: &ast::Instruction = stmt.into();
        *addr = match &instruction.instruction {
            ast::InstructionType::Add(mem_location) => {
                ASSEMBLED_OPCODE_ADD + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::Subtract(mem_location) => {
                ASSEMBLED_OPCODE_SUB + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::Store(mem_location) => {
                ASSEMBLED_OPCODE_STA + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::Load(mem_location) => {
                ASSEMBLED_OPCODE_LDA + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::BranchAlways(mem_location) => {
                ASSEMBLED_OPCODE_BRA + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::BranchIfZero(mem_location) => {
                ASSEMBLED_OPCODE_BRZ + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::BranchIfPositive(mem_location) => {
                ASSEMBLED_OPCODE_BRP + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::Input => ASSEMBLED_OPCODE_INP,
            ast::InstructionType::Output => ASSEMBLED_OPCODE_OUT,
            ast::InstructionType::Halt => ASSEMBLED_OPCODE_HLT,
            ast::InstructionType::Data(v) => *v,
        }
    }

    Ok(())
}
