use std::env;

mod lexer;
mod compilation_unit;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("feed some arguments for compiler!\n example: ./suco <source path> <output name>")
    }
    compilation_unit::compilation_unit(args);
}