use std::fs;
use std::process::Command;

use crate::lexer::Lexer;
use crate::lexer::token::Token;

#[derive(Debug)]
pub enum CompileFlag {
    Lex,
    Parse,
    Codgen,
}

pub fn preproccess(in_path: &str, out_path: &str) {
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

// TODO(Thomas): Doesn't produce an assembly file yet
// TODO(Thomas): This should take a CompileFlag and do actions accordingly
pub fn compile(in_path: &str) {
    println!("Compiling!");

    // read contents of the preprocessed file
    // lex the contents
    let source: String = fs::read_to_string(in_path).unwrap();
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = Lexer::new(source.as_str());
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("tokens: {:?}", tokens);

    fs::remove_file(in_path).expect("failed to remove file");
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

pub fn parse_flag(input: &str) -> CompileFlag {
    match input {
        "--lex" => CompileFlag::Lex,
        "--parse" => CompileFlag::Parse,
        "--codegen" => CompileFlag::Codgen,
        _ => panic!("illegal flag"),
    }
}
