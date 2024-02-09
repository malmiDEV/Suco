use std::fs::File;
use std::io::{Read, Write};
use std::process;
use std::process::{Command, exit};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::parser::{Parser, Types, NodeKind, IntType, Node};

pub fn compilation_unit(args: Vec<String>) {
    let file_name = &args[1];
    let output = &args[2];
    let mut file = File::open(file_name).unwrap();
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
        let nod = &node[i];
        if nod.typ == Types::Function {
            match &nod.kind {
                NodeKind::Function(name, typ, param, body) => {
                    println!("{:?}", param);
                    asm.push_str(format!("{}:\n",name).as_str());
                    match &body.kind {
                        NodeKind::Scope(a) => {
                            asm.push_str("\tpush ebp\
                                        \n\tmov ebp, esp\n");
                            for i in 0..a.len() {
                                match &a[i].kind {
                                    NodeKind::Return(a) => match a.kind {
                                        NodeKind::Number(n) => {
                                            let mut reg = String::new();
                                            let verified = match typ {
                                                IntType::Int8   => match n {
                                                    -0x80..=0x7f => {
                                                        reg.push_str("al");
                                                        n
                                                    },
                                                    _ => panic!("Value Not Fit into i8 . {} but range is -128 .. 127", n)
                                                }
                                                IntType::Uint8  => match n {
                                                    0x00..=0xff => {
                                                        reg.push_str("al");
                                                        n
                                                    }
                                                    _ => panic!("Value Not Fit into u8 . {} but range is 0 .. 255", n)
                                                },
                                                IntType::Int16  => match n {
                                                    -0x8000..=0x7fff => {
                                                        reg.push_str("ax");
                                                        n
                                                    }
                                                    _ => panic!("Value Not Fit into i16. {} but range is -32,768 .. 32,767", n)
                                                },
                                                IntType::Uint16 => match n {
                                                    0x0000..=0xffff => {
                                                        reg.push_str("ax");
                                                        n
                                                    }
                                                    _ => panic!("Value Not Fit into u16. {} but range is 0 .. 65,535", n)
                                                },
                                                IntType::Int32  => match n {
                                                    -0x80000000..=0x7fffffff => {
                                                        reg.push_str("eax");
                                                        n
                                                    }
                                                    _ => panic!("Value Not Fit into i32. {} but range is -2,147,483,648 .. 2,147,483,647", n)
                                                },
                                                IntType::Uint32 => match n {
                                                    0x00000000..=0xffffffff => {
                                                        reg.push_str("eax");
                                                        n
                                                    }
                                                    _ => panic!("Value Not Fit Into u32. {} but range is 0 .. 2,147,483,647", n)
                                                },
                                                _ => panic!("Not Supported Type: {:?}", typ)
                                            };
                                            // let size = match &a.typ {
                                            //     Types::Int(a) => match a {
                                            //         IntType::Int8 => "byte",
                                            //         IntType::Uint8 => "word",
                                            //         IntType::Int16 => "word",
                                            //         IntType::Uint16 => "word",
                                            //         IntType::Int32 => "dword",
                                            //         IntType::Int64 => "qword",
                                            //         _ => ""
                                            //     }
                                            //     _ => todo!()
                                            // };
                                            let template = format!("\tmov {}, {}\n",reg, verified);
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
            asm.push_str("\tmov esp, ebp\
                        \n\tpop ebp\
                        \n\tret")
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
        Ok(_) => println!("Compiled to :: Path({}) as :: Output({})\n{} Byte => {}\n",
                          file_name, output, file_name, buffer.len()),
        Err(err) => eprintln!("Error writing to asm file: {}", err),
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