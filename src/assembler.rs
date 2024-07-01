use std::collections::HashMap;

use crate::ast::{self, Statement};

#[derive(Debug)]
pub enum AssemblerError<'a> {
    TooManyInstructions { expected: usize, actual: usize },
    LabelAlreadyDefined { name: &'a str, index: usize },
    LabelNotDefined(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub enum AssembledInstruction {
    Add(u8),
    Subtract(u8),
    Store(u8),
    Load(u8),
    BranchAlways(u8),
    BranchIfZero(u8),
    BranchIfPositive(u8),
    Input,
    Output,
    Halt,
    Data(u16),
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
    memory: &mut [AssembledInstruction; 255],
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
                AssembledInstruction::Add(memory_location_to_addr(&labels, mem_location)?)
            }
            ast::InstructionType::Subtract(mem_location) => {
                AssembledInstruction::Subtract(memory_location_to_addr(&labels, mem_location)?)
            }
            ast::InstructionType::Store(mem_location) => {
                AssembledInstruction::Store(memory_location_to_addr(&labels, mem_location)?)
            }
            ast::InstructionType::Load(mem_location) => {
                AssembledInstruction::Load(memory_location_to_addr(&labels, mem_location)?)
            }
            ast::InstructionType::BranchAlways(mem_location) => {
                AssembledInstruction::BranchAlways(memory_location_to_addr(&labels, mem_location)?)
            }
            ast::InstructionType::BranchIfZero(mem_location) => {
                AssembledInstruction::BranchIfZero(memory_location_to_addr(&labels, mem_location)?)
            }
            ast::InstructionType::BranchIfPositive(mem_location) => {
                AssembledInstruction::BranchIfPositive(memory_location_to_addr(
                    &labels,
                    mem_location,
                )?)
            }
            ast::InstructionType::Input => AssembledInstruction::Input,
            ast::InstructionType::Output => AssembledInstruction::Output,
            ast::InstructionType::Halt => AssembledInstruction::Halt,
            ast::InstructionType::Data(v) => AssembledInstruction::Data(*v),
        }
    }

    Ok(())
}
