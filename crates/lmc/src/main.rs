use std::path::PathBuf;

use clap::{Parser, Subcommand};
use lmc_core::assembler::assemble_from_ast;
use lmc_core::ast::parsed_to_ast;
use lmc_core::grammar::pass_program;
use lmc_core::runtime::{CommandLine, Runtime};

#[derive(Subcommand, Debug)]
enum Command {
    /// Show friendly outputs of internal representations
    Show {
        /// Show original source code
        #[arg(long = "source")]
        show_source: bool,
        /// Show tokenized source code
        #[arg(long = "tokenized")]
        show_tokenized: bool,
        /// Show the Abstract Syntax Tree of source code
        #[arg(long = "ast")]
        show_ast: bool,
        /// Show final assembled output of source code
        #[arg(long = "assembled")]
        show_assembled: bool,
        /// Enable all outputs
        #[arg(long = "all")]
        show_all: bool,
    },
    /// Run the LMC code, using a CLI environment
    Run,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// LMC code file to process
    #[arg(short = 'f', long = "file")]
    pub file_path: PathBuf,
    #[command(subcommand)]
    pub command: Command,
}

fn main() {
    let args = Args::parse();

    let file_content = std::fs::read_to_string(args.file_path).unwrap();

    let mut parsed = pass_program(&file_content).unwrap();
    let tokens = parsed.clone();
    let ast = parsed_to_ast(&mut parsed);
    let mut assembled = [0; 100];
    assemble_from_ast(&ast, &mut assembled).unwrap();

    match args.command {
        Command::Show {
            show_source,
            show_tokenized,
            show_ast,
            show_assembled,
            show_all,
        } => {
            if show_source || show_all {
                println!("--- Source ---\n{}\n--- END ---", file_content);
            }
            if show_tokenized || show_all {
                println!("--- Tokenized ---\n{:?}\n--- END ---", tokens);
            }
            if show_ast || show_all {
                println!("--- Abstract Syntax Tree (AST) ---\n{:?}\n--- END ---", ast);
            }
            if show_assembled || show_all {
                println!("--- Assembled ---\n{:?}\n--- END ---", assembled);
            }
        }
        Command::Run => CommandLine::load_assembled(&mut assembled).run(),
    }
}
