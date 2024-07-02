use pest::{error::Error as PestError, iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LMCParser;

#[allow(clippy::result_large_err)]
pub fn pass_program(input: &str) -> Result<Pairs<Rule>, PestError<Rule>> {
    LMCParser::parse(Rule::program, input)
}

#[cfg(test)]
mod tests {
    use super::{LMCParser, Rule};
    use pest::Parser;

    #[test]
    fn test_comment() {
        LMCParser::parse(Rule::comment, ";a comment").unwrap();
        LMCParser::parse(Rule::comment, "; a comment").unwrap();
        assert!(LMCParser::parse(Rule::comment, "").is_err());
    }

    #[test]
    fn test_label() {
        LMCParser::parse(Rule::label, "myLabel:").unwrap();
        LMCParser::parse(Rule::label, "; comment one\nmyLabel:").unwrap();
        LMCParser::parse(Rule::label, "myLabel:; comment two").unwrap();
        assert!(LMCParser::parse(Rule::label, "myLabel another:").is_err());
        assert!(LMCParser::parse(Rule::label, "myLabel").is_err());
    }

    #[test]
    fn test_memory_location() {
        LMCParser::parse(Rule::memoryLocation, "10").unwrap();
        LMCParser::parse(Rule::memoryLocation, "0").unwrap();
        LMCParser::parse(Rule::memoryLocation, "labelled").unwrap();
        assert!(LMCParser::parse(Rule::memoryLocation, "").is_err());
    }

    #[test]
    fn test_instruction_name() {
        LMCParser::parse(Rule::instruction, "ADD").unwrap();
        LMCParser::parse(Rule::instruction, "ADD ").unwrap();
        LMCParser::parse(Rule::instruction, "add").unwrap();
        assert!(LMCParser::parse(Rule::instruction, "ADDDD").is_err());
        assert!(LMCParser::parse(Rule::instruction, "").is_err());
    }

    #[test]
    fn test_intruction() {
        LMCParser::parse(Rule::instruction, "ADD var").unwrap();
        LMCParser::parse(Rule::instruction, "ADD var ; add something").unwrap();
        LMCParser::parse(Rule::instruction, "add: ADD var").unwrap();
        LMCParser::parse(Rule::instruction, "add:\n ADD var").unwrap();
    }

    #[test]
    fn test_program() {
        LMCParser::parse(
            Rule::program,
            r#"
; get input
start:INP
STA 99
INP
ADD 99
OUT
HLT
; Output the sum of two numbers
        "#,
        )
        .unwrap();
    }
}
