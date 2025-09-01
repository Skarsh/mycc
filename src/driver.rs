use std::fs;
use std::process::Command;

#[derive(Debug)]
pub enum Flag {
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

// TODO: Just STUBBED out for now
pub fn compile(in_path: &str) {
    println!("Compiling!");
    // TODO: Add our compilation step here
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

pub fn parse_flag(input: &str) -> Flag {
    match input {
        "--lex" => Flag::Lex,
        "--parse" => Flag::Parse,
        "--codegen" => Flag::Codgen,
        _ => panic!("illegal flag"),
    }
}
