mod driver;
mod lexer;

use crate::driver::{CompileFlag, Compiler, parse_flag};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", env::current_dir().unwrap());

    let src_file_path;
    let mut flag: CompileFlag = CompileFlag::Codegen;
    if args.len() > 2 {
        src_file_path = &args[2];
        flag = parse_flag(&args[1]);
    } else if args.len() > 1 {
        src_file_path = &args[1];
    } else {
        panic!("No source file given")
    }

    let mut compiler = Compiler::new();
    compiler.run(src_file_path, flag);
}
