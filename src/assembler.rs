use std::collections::HashMap;

use crate::ast::{self, Statement};

pub const OPCODE_ADD: usize = 100;
pub const OPCODE_SUB: usize = 200;
pub const OPCODE_STA: usize = 300;
pub const OPCODE_LDA: usize = 500;
pub const OPCODE_BRA: usize = 600;
pub const OPCODE_BRZ: usize = 700;
pub const OPCODE_BRP: usize = 800;
pub const OPCODE_INP: usize = 901;
pub const OPCODE_OUT: usize = 902;
pub const OPCODE_HLT: usize = 000;

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
                OPCODE_ADD + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::Subtract(mem_location) => {
                OPCODE_SUB + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::Store(mem_location) => {
                OPCODE_STA + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::Load(mem_location) => {
                OPCODE_LDA + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::BranchAlways(mem_location) => {
                OPCODE_BRA + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::BranchIfZero(mem_location) => {
                OPCODE_BRZ + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::BranchIfPositive(mem_location) => {
                OPCODE_BRP + memory_location_to_addr(&labels, mem_location)? as usize
            }
            ast::InstructionType::Input => OPCODE_INP,
            ast::InstructionType::Output => OPCODE_OUT,
            ast::InstructionType::Halt => OPCODE_BRZ,
            ast::InstructionType::Data(v) => *v,
        }
    }

    Ok(())
}
