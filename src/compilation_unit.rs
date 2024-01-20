use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use crate::lexer::{Lexer, Token, TokenKind};
use crate::parser::{Parser, Types, NodeKind};

pub fn compilation_unit(args: Vec<String>) {
    let file = &args[1];
    let output = &args[2];
    let mut file = File::open(file).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("file cannot open");

    let mut lex = Lexer::new(buffer.as_str());
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(c) = lex.tokenize() {
        if c.kind == TokenKind::Eof {
            tokens.push(c);
            break;
        }
        if c.kind == TokenKind::WhiteSpace {
            continue
        }
        tokens.push(c);
    }

    let mut parse = Parser::new(&tokens);
    let node = parse.parsing_unit();
    let mut asm = String::new();
    for i in 0..node.len() {
        let nod = match &node[i] {
            Ok(a) => a,
            Err(e) => panic!("{:?}",e)
        };
        if nod.typ == Types::Function {
            match &nod.kind {
                NodeKind::Function {
                    name, typ, body
                } => {
                    let template = format!("{}:\n\tpush ebp\n\tmov ebp, esp\n", name);
                    asm.push_str(template.as_str());
                    match &body.kind {
                        NodeKind::Scope(a) => {
                            for i in 0..a.len() {
                                match &a[i].kind {
                                    NodeKind::Return(a) => match a.kind {
                                        NodeKind::Number(n) => {
                                            let template = format!("\tmov eax, {}\n", n);
                                            asm.push_str(template.as_str())
                                        },
                                        _ => todo!()
                                    },
                                    _ => todo!()
                                };
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            };
            asm.push_str("\tmov esp, ebp\n\tpop ebp\n\tret")
        } else {
            panic!();
        }
    }
    let mut file = match File::create("./main.asm") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating file: {}", err);
            return;
        }
    };
    match file.write_all(asm.as_bytes()) {
        Ok(_) => println!("Data has been successfully saved to the file."),
        Err(err) => eprintln!("Error writing to file: {}", err),
    }
    Command::new("nasm")
        .arg("main.asm")
        .arg("-f")
        .arg("bin")
        .arg("-o")
        .arg(output)
        .status()
        .unwrap();
}