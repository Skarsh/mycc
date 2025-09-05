use std::fs;
use std::process::Command;

use crate::lexer::Lexer;
use crate::lexer::token::Token;

#[derive(Debug)]
pub enum CompileFlag {
    Lex,
    Parse,
    Codegen,
}

pub struct Compiler {
    lexer: Lexer,
    tokens: Vec<Token>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            lexer: Lexer::new(""),
            tokens: Vec::new(),
        }
    }

    pub fn run(&mut self, source_path: &str, flag: CompileFlag) {
        let preprocessed_out_path = "../preprocessed.i";
        Self::preprocess(source_path, preprocessed_out_path);

        let preprocessed_source_contents = fs::read_to_string(preprocessed_out_path).unwrap();

        self.lexer = Lexer::new(&preprocessed_source_contents);
        self.compile(flag);

        fs::remove_file(preprocessed_out_path).expect("failed to remove file");

        Self::assemble_and_link(".s", "");
    }

    pub fn preprocess(in_path: &str, out_path: &str) {
        println!("Preprocssing!");
        let output = Command::new("gcc")
            .args(["-E", "-P", in_path, "-o", out_path])
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            eprintln!("Preproccesing failed.\n{}\n{}", output.status, error_msg);
        }
    }

    pub fn compile(&mut self, flag: CompileFlag) {
        match flag {
            CompileFlag::Lex => {
                while let Some(token) = self.lexer.next_token() {
                    self.tokens.push(token);
                }
                println!("tokens: {:?}", self.tokens)
            }
            CompileFlag::Parse => todo!(),
            CompileFlag::Codegen => todo!(),
        }
    }

    pub fn assemble_and_link(in_path: &str, out_path: &str) {
        println!("Assembling and linking!");
        let output = Command::new("gcc")
            .args([in_path, "-o", out_path])
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            eprintln!(
                "Assemble and linking failed.\n {}\n{}",
                output.status, error_msg
            )
        }
    }
}

pub fn parse_flag(input: &str) -> CompileFlag {
    match input {
        "--lex" => CompileFlag::Lex,
        "--parse" => CompileFlag::Parse,
        "--codegen" => CompileFlag::Codegen,
        _ => panic!("illegal flag"),
    }
}
