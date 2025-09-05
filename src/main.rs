mod driver;
mod lexer;

use crate::driver::{CompileFlag, assemble_and_link, compile, parse_flag, preproccess};
use crate::lexer::Lexer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", env::current_dir().unwrap());

    let src_file_path = &args[1];

    let mut flag: Option<CompileFlag> = None;
    if args.len() > 2 {
        flag = Some(parse_flag(&args[2]));
    }

    let preprocessed_out_path = "../preprocessed.i";

    // Preprocess
    preproccess(src_file_path, preprocessed_out_path);

    // Compile preprocessed file
    compile(preprocessed_out_path);

    // Assemble and link
    assemble_and_link(".s", "");
}
