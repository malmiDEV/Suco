use std::fs::File;
use std::io::{Read, Write};
use std::process;
use std::process::{Command, exit};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::parser::{Parser, Types, NodeKind, IntType, Node};

pub fn compilation_unit(args: Vec<String>) {
    // let file_name = &args[1];
    // let output = &args[2];
    let file_name = "main.su";
    let output = "main.bin";
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

    let mut epilogue = String::new();
    let mut content = String::new();
    let mut stack_alloc = 16;
    let mut stack_size_base = 0;
    for i in 0..node.len() {
        let nod = &node[i];
        if nod.typ == Types::Function {
            match &nod.kind {
                NodeKind::Function(name, typ, param, body) => {
                    // match &param.kind {
                    //     NodeKind::Param(v) => println!("{:?}",v[0]),
                    //     _ => {}
                    // }

                    asm.push_str(format!("{}:\n",name).as_str());
                    match &body.kind {
                        NodeKind::Scope(a) => {
                            epilogue.clear();
                            content.clear();
                            stack_alloc = 16;
                            stack_size_base = 0;
                            epilogue.push_str("\tpush ebp\n\
                                               \tmov ebp, esp\n");
                            for i in 0..a.len() {
                                match &a[i].kind {
                                    NodeKind::Return(a) => {
                                        let mut reg = String::new();
                                        let mut verified;
                                        match a.kind {
                                            NodeKind::NumberLit(n) => {
                                                verified = match typ {
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
                                            },
                                            _ => todo!()
                                        }
                                        content.push_str(format!("\tmov {}, {}\n",reg, verified).as_str());
                                    },
                                    NodeKind::Variable(v) => {
                                        let type_ = match v.types {
                                            IntType::Int8  => match v.content {
                                                -0x80..=0x7f => "byte",
                                                _ => panic!("Value of '{}' Not Fit into i8. {} but range is -128 .. 127", v.name, v.content)
                                            }
                                            IntType::Uint8 => match v.content {
                                                0x00..=0xff => "byte",
                                                _ => panic!("Value of '{}' Not Fit into u8. {} but range is 0 .. 255", v.name, v.content)
                                            },
                                            IntType::Int16 => match v.content {
                                                -0x8000..=0x7fff => "word",
                                                _ => panic!("Value of '{}' Not Fit into i16. {} but range is -32,768 .. 32,767", v.name, v.content)
                                            },
                                            IntType::Uint16=> match v.content {
                                                0x0000..=0xffff => "word",
                                                _ => panic!("Value of '{}' Not Fit into u16. {} but range is 0 .. 65,535", v.name, v.content)
                                            },
                                            IntType::Int32 => match v.content {
                                                -0x80000000..=0x7fffffff => "dword",
                                                _ => panic!("Value of '{}' Not Fit into i32. {} but range is -2,147,483,648 .. 2,147,483,647", v.name, v.content)
                                            },
                                            IntType::Uint32=> match v.content {
                                                0x00000000..=0xffffffff => "dword",
                                                _ => panic!("Value of '{}' Not Fit Into u32. {} but range is 0 .. 2,147,483,647", v.name, v.content)
                                            },
                                            _ => panic!("Not Supported Type {:?}", v.types)
                                        };
                                        let stack_size = match type_ {
                                            "byte" => {
                                                1
                                            },
                                            "word" => {
                                                2
                                            },
                                            "dword" => {
                                                4
                                            },
                                            _ => panic!("Not Supported Size {}", type_)
                                        };
                                        let size = stack_size_base;
                                        stack_size_base = (size + stack_size - 1) & !(stack_size - 1);
                                        stack_size_base += stack_size;
                                        let base = 16;
                                        stack_alloc = (stack_size_base + base - 1) & !(base - 1);
                                        let template = format!("\tmov {} [ebp-{}], {}\n", type_, stack_size_base, v.content);
                                        content.push_str(&template);
                                    }
                                    _ => todo!()
                                };
                            }
                            epilogue.push_str(format!("\tsub esp, {}\n", stack_alloc).as_str());
                            asm.push_str(epilogue.as_str());
                            asm.push_str(content.as_str());
                            let template = "\tmov esp, ebp\n\
                                            \tpop ebp\n\
                                            \tret\n";
                            asm.push_str(template)
                        },
                        _ => {}
                    }
                },
                _ => {}
            };
        } else {
            panic!();
        }
    }
    println!("{}", asm);
    // let mut file = match File::create("./main.asm") {
    //     Ok(file) => file,
    //     Err(err) => {
    //         eprintln!("Error creating file: {}", err);
    //         return;
    //     }
    // };
    // match file.write_all(asm.as_bytes()) {
    //     Ok(_) => println!("Compiled to :: Path({}) as :: Output({})\n{} Byte => {}\n",
    //                       file_name, output, file_name, buffer.len()),
    //     Err(err) => eprintln!("Error writing to asm file: {}", err),
    // }
}
