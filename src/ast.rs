use pest::iterators::Pairs;

use crate::grammar::Rule;

#[derive(Debug)]
pub enum MemoryLocation<'a> {
    Address(u8),
    Label(&'a str),
}

#[derive(Debug)]
pub enum InstructionType<'a> {
    Add(MemoryLocation<'a>),
    Subtract(MemoryLocation<'a>),
    Store(MemoryLocation<'a>),
    Load(MemoryLocation<'a>),
    BranchAlways(MemoryLocation<'a>),
    BranchIfZero(MemoryLocation<'a>),
    BranchIfPositive(MemoryLocation<'a>),
    Input,
    Output,
    Halt,
    Data(isize),
}

#[derive(Debug)]
pub struct Label<'a> {
    pub label: &'a str,
    pub comments: Box<[&'a str]>,
}

#[derive(Debug)]
pub struct Instruction<'a> {
    pub instruction: InstructionType<'a>,
    pub comments: Box<[&'a str]>,
}

#[derive(Debug)]
pub enum Statement<'a> {
    Labeled {
        label: Label<'a>,
        instruction: Instruction<'a>,
    },
    UnLabeled {
        instruction: Instruction<'a>,
    },
}

impl<'a, 'b> From<&'b Statement<'a>> for &'b Instruction<'a> {
    fn from(value: &'b Statement<'a>) -> &'b Instruction<'a> {
        match value {
            Statement::Labeled { instruction, .. } => &instruction,
            Statement::UnLabeled { instruction } => &instruction,
        }
    }
}

pub fn parsed_to_ast<'a>(parsed: &mut Pairs<'a, Rule>) -> Vec<Statement<'a>> {
    let mut ast = vec![];
    for pair in parsed {
        let rule = pair.as_rule();
        match rule {
            Rule::stmt => {
                let tokens = pair.into_inner();
                let mut label = None;
                let mut instruction = None;
                for token in tokens {
                    match token.as_rule() {
                        Rule::label => {
                            let mut label_comments = vec![];
                            for token in token.into_inner() {
                                match token.as_rule() {
                                    Rule::labelName => {
                                        label = Some(Label {
                                            label: token.as_span().as_str(),
                                            comments: vec![].into_boxed_slice(),
                                        })
                                    }
                                    Rule::comment => {
                                        let v = token
                                            .as_span()
                                            .as_str()
                                            .strip_prefix(';')
                                            .unwrap()
                                            .trim();
                                        label_comments.push(v);
                                    }
                                    _ => panic!("invalid parsed token rule"),
                                }
                            }
                            if let Some(label) = label.as_mut() {
                                label.comments = label_comments.into_boxed_slice();
                            }
                        }
                        Rule::instruction => {
                            let mut instruction_comments = vec![];
                            let mut instruction_type = String::new();
                            let mut instruction_memory = "";
                            for token in token.into_inner() {
                                match token.as_rule() {
                                    Rule::instructionName => {
                                        instruction_type = token.as_span().as_str().to_uppercase();
                                    }
                                    Rule::memoryLocation => {
                                        instruction_memory = token.as_span().as_str();
                                    }
                                    Rule::comment => {
                                        let v = token
                                            .as_span()
                                            .as_str()
                                            .strip_prefix(';')
                                            .unwrap()
                                            .trim();
                                        instruction_comments.push(v);
                                    }
                                    _ => panic!("invalid parsed token rule"),
                                }
                            }
                            let memory_location = match (
                                instruction_memory.parse::<u8>(),
                                instruction_memory.is_empty(),
                            ) {
                                (Ok(addr), false) => MemoryLocation::Address(addr),
                                (Err(_), false) => MemoryLocation::Label(instruction_memory),
                                (_, true) => MemoryLocation::Address(0),
                            };
                            instruction = Some(Instruction {
                                instruction: match &*instruction_type {
                                    "ADD" => InstructionType::Add(memory_location),
                                    "SUB" => InstructionType::Subtract(memory_location),
                                    "STA" => InstructionType::Store(memory_location),
                                    "LDA" => InstructionType::Load(memory_location),
                                    "BRA" => InstructionType::BranchAlways(memory_location),
                                    "BRP" => InstructionType::BranchIfPositive(memory_location),
                                    "INP" => InstructionType::Input,
                                    "OUT" => InstructionType::Output,
                                    "HLT" => InstructionType::Halt,
                                    "DAT" => InstructionType::Data(
                                        instruction_memory.parse::<isize>().unwrap_or(0),
                                    ),
                                    _ => panic!(
                                        "unknown instruction type found: '{}'",
                                        instruction_type
                                    ),
                                },
                                comments: instruction_comments.into_boxed_slice(),
                            });
                        }
                        _ => panic!("invalid parsed token rule"),
                    }
                }
                ast.push(match label {
                    Some(label) => Statement::Labeled {
                        label,
                        instruction: instruction.unwrap(),
                    },
                    None => Statement::UnLabeled {
                        instruction: instruction.unwrap(),
                    },
                })
            }
            Rule::comment => (),
            Rule::EOI => break,
            _ => panic!("invalid parsed rules"),
        }
    }
    ast
}
