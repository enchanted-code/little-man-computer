use pest::iterators::Pairs;

use crate::grammar::Rule;

pub const MNEMONIC_ADD: &str = "ADD";
pub const MNEMONIC_SUB: &str = "SUB";
pub const MNEMONIC_STA: &str = "STA";
pub const MNEMONIC_LDA: &str = "LDA";
pub const MNEMONIC_BRA: &str = "BRA";
pub const MNEMONIC_BRZ: &str = "BRZ";
pub const MNEMONIC_BRP: &str = "BRP";
pub const MNEMONIC_INP: &str = "INP";
pub const MNEMONIC_OUT: &str = "OUT";
pub const MNEMONIC_HLT: &str = "HLT";
pub const MNEMONIC_DAT: &str = "DAT";

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryLocation<'a> {
    Address(u8),
    Label(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
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
    Data(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Label<'a> {
    pub label: &'a str,
    pub comments: Box<[&'a str]>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction<'a> {
    pub instruction: InstructionType<'a>,
    pub comments: Box<[&'a str]>,
}

#[derive(Debug, PartialEq, Eq)]
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
                                    MNEMONIC_ADD => InstructionType::Add(memory_location),
                                    MNEMONIC_SUB => InstructionType::Subtract(memory_location),
                                    MNEMONIC_STA => InstructionType::Store(memory_location),
                                    MNEMONIC_LDA => InstructionType::Load(memory_location),
                                    MNEMONIC_BRA => InstructionType::BranchAlways(memory_location),
                                    MNEMONIC_BRZ => InstructionType::BranchIfZero(memory_location),
                                    MNEMONIC_BRP => {
                                        InstructionType::BranchIfPositive(memory_location)
                                    }
                                    MNEMONIC_INP => InstructionType::Input,
                                    MNEMONIC_OUT => InstructionType::Output,
                                    MNEMONIC_HLT => InstructionType::Halt,
                                    MNEMONIC_DAT => InstructionType::Data(
                                        instruction_memory.parse::<usize>().unwrap_or(0),
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

#[cfg(test)]
mod tests {
    use crate::grammar::pass_program;

    use super::{parsed_to_ast, Instruction, InstructionType, Label, MemoryLocation, Statement};

    #[test]
    fn test_simple_add() {
        let mut parsed = pass_program(
            r#"
; add two numbers
start:
    LDA a
    ADD b
    OUT
a: DAT 2
b: DAT 4
"#,
        )
        .unwrap();

        let ast_actual = parsed_to_ast(&mut parsed);
        let ast_expected = vec![
            Statement::Labeled {
                label: Label {
                    label: "start",
                    comments: Box::new(["add two numbers"]),
                },
                instruction: Instruction {
                    instruction: InstructionType::Load(MemoryLocation::Label("a")),
                    comments: Box::new([]),
                },
            },
            Statement::UnLabeled {
                instruction: Instruction {
                    instruction: InstructionType::Add(MemoryLocation::Label("b")),
                    comments: Box::new([]),
                },
            },
            Statement::UnLabeled {
                instruction: Instruction {
                    instruction: InstructionType::Output,
                    comments: Box::new([]),
                },
            },
            Statement::Labeled {
                label: Label {
                    label: "a",
                    comments: Box::new([]),
                },
                instruction: Instruction {
                    instruction: InstructionType::Data(2),
                    comments: Box::new([]),
                },
            },
            Statement::Labeled {
                label: Label {
                    label: "b",
                    comments: Box::new([]),
                },
                instruction: Instruction {
                    instruction: InstructionType::Data(4),
                    comments: Box::new([]),
                },
            },
        ];

        assert_eq!(ast_expected, ast_actual);
    }
}
