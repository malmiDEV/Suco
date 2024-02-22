use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::process::exit;
use crate::lexer::{Token, TokenKind};
use crate::parser::NodeKind::Param;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum IntType {
    Uint0,
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
}

impl IntType {
    pub fn size(&self) -> u32 {
        match self {
            IntType::Uint0 => 0,
            IntType::Int8 => 1,
            IntType::Uint8 => 1,
            IntType::Int16 => 2,
            IntType::Uint16 => 2,
            IntType::Int32 => 4,
            IntType::Uint32 => 4,
            IntType::Int64 => 8,
            IntType::Uint64 => 8,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Types {
    Int(IntType),
    Function
}

impl Types {
    pub fn detect(&self) -> u32 {
        match self {
            Types::Int(int) => int.size(),
            Types::Function => 0
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum NodeKind {
    Program,
    Annotation,
    NumberLit(i64),
    String(usize),
    Identifier(String),
    Variable(Box<Variable>),
    Param(Vec<Parameter>),
    Scope(Vec<Node>),
    Function(String, IntType, Box<Node>, Box<Node>),
    Return(Box<Node>)
}

#[derive(PartialEq, Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub types: IntType,
    pub content: u32,
}

impl Parameter {
    pub fn new(name: String, types: IntType, content: u32) -> Parameter {
        Parameter {
            name,
            types,
            content,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub types: IntType,
    pub content: i64,
    pub global: bool
}

impl Variable {
    pub fn new(name: String, types: IntType, content: i64, global: bool) -> Variable {
        Variable {
            name,
            types,
            content,
            global
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub typ: Types
}

impl Node {
    pub fn new() -> Self {
        Self {
            kind: NodeKind::Program,                         // default state
            typ: Types::Int(IntType::Int32)            // default integer type
        }
    }

    fn new_int(val: i64, typ: IntType) -> Self {
        Self {
            kind: NodeKind::NumberLit(val),
            typ: Types::Int(typ)
        }
    }

    fn new_return(val: Self) -> Self {
        Self {
            kind: NodeKind::Return(Box::new(val)),
            typ: Types::Int(IntType::Uint0),
        }
    }

    fn new_identifier(val: String) -> Self {
        Self {
            kind: NodeKind::Identifier(val),
            typ: Types::Int(IntType::Uint0)
        }
    }

    fn new_variable(args: Variable) -> Self {
        Self {
            kind: NodeKind::Variable(Box::new(args)),
            typ: Types::Int(IntType::Uint0)
        }
    }

    fn new_params(args: Vec<Parameter>) -> Self {
        Self {
            kind: NodeKind::Param(args),
            typ: Types::Int(IntType::Uint0)
        }
    }

    fn new_scope(statements: Vec<Node>) -> Self {
        Self {
            kind: NodeKind::Scope(statements),
            typ: Types::Int(IntType::Uint0)
        }
    }

    fn new_function(name: &String, typ: IntType, param: Self, block: Self) -> Self  {
        let name = name.to_string();
        Self {
            kind: NodeKind::Function(name, typ, Box::new(param), Box::new(block)),
            typ: Types::Function
        }
    }
}

type ParseResult<T> = Result<T, String>;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pos: usize
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0
        }
    }

    pub fn parsing_unit(&mut self) -> Vec<Node> {
        let mut r = Vec::new();

        while let Some(token) = self.tokens.get(self.pos) {
            let chr = token.span.chars().next();
            if let Some(c) = chr {
                if !c.is_ascii_punctuation() {
                    let span = token.span.as_str();
                    match span {
                        "defun" => match self.parse_func() {
                            Ok(d) => r.push(d),
                            Err(e) => panic!("{:?}", e)
                        },
                        "\0" => {},
                        _ => {
                            panic!("Unexpected Keyword: {:?}", span);
                        }
                    }
                }
            }
            self.pos += 1;
        }
        r
    }

    pub fn parse_func(&mut self) -> ParseResult<Node> {
        let identifier = self.parse_identifier()?;
        let parameter = self.parse_params()?;
        let fn_type = self.parse_fn_type()?;
        let fn_body = self.parse_scope(fn_type.typ.clone())?;

        let name = match identifier.kind {
            NodeKind::Identifier(a) => a,
            _ => todo!()
        };

        let typ = match fn_type.typ {
            Types::Int(a) => a,
            _ => todo!()
        };

        Ok(Node::new_function(&name, typ, parameter, fn_body))
    }

    pub fn parse_return(&mut self, typ: Types) -> ParseResult<Node> {
        let expr = self.parse_expr(typ)?;
        let _ = self.consume_semi()?;
        Ok(Node::new_return(expr))
    }

    pub fn parse_identifier(&mut self) -> ParseResult<Node> {
        self.pos += 1;
        if let Some(x) = self.tokens.get(self.pos) {
            match x {
                Token { kind: TokenKind::Identifier, .. } => {
                    match x.span.parse() {
                        Ok(n) => Ok(Node::new_identifier(n)),
                        Err(_) => Err(format!("{}", "Missing Expr".to_string()))
                    }
                },
                _ => Err(format!("Unexpected Token {}", x.span))
            }
        } else {
            Err(format!("{}", "Missing Token".to_string()))
        }
    }

    pub fn parse_params(&mut self) -> ParseResult<Node> {
        self.pos += 1;
        if let Some(p) = self.tokens.get(self.pos) {
            if p.kind != TokenKind::Lparen {
                return Err(format!("Expected ( but found::{}", p.span))
            }
            self.pos += 1;
        }
        let mut params: Vec<Parameter> = Vec::new();
        while let Some(i) = self.tokens.get(self.pos) {
            match i.kind {
                TokenKind::Rparen => break,
                TokenKind::Comma => {},
                TokenKind::Identifier => {
                    let mut name = String::new();
                    name.push_str(i.span.as_str());
                    self.pos += 1;
                    if let Some(colon) = self.tokens.get(self.pos) {
                        if colon.kind == TokenKind::Colon {
                            self.pos += 1;
                            if let Some(t) = self.tokens.get(self.pos) {
                                params.push(Parameter::new(name, match t.kind {
                                    TokenKind::Uint8 => IntType::Uint8,
                                    TokenKind::Int8 => IntType::Int8,
                                    TokenKind::Uint16 => IntType::Uint16,
                                    TokenKind::Int16 => IntType::Int16,
                                    TokenKind::Uint32 => IntType::Uint32,
                                    TokenKind::Int32 => IntType::Int32,
                                    _ => return Err(format!("Type Not Exist Error AT Token: {} Number: {}", t.span, self.pos))
                                }, 0))
                            }
                        } else {
                            return Err(format!("Unexpected Token AT Token After: {} Number: {} expected :", i.span, self.pos))
                        }
                    }
                },
                _ => return Err(format!("Unexpected Token AT Token: {} Number: {}", i.span, self.pos))
            }
            self.pos += 1;
        }
        Ok(Node::new_params(params))
    }

    pub fn parse_variable(&mut self) -> ParseResult<Node> {
        self.pos += 1;
        let mut name = String::new();
        let mut tp = IntType::Int32;
        let mut expr = Node::new_int(0, IntType::Int32);
        if let Some(id) = self.tokens.get(self.pos) {
            match id.kind {
                TokenKind::Identifier => {
                    self.pos += 1;
                    name.push_str(id.span.as_str());
                    if let Some(t) = self.tokens.get(self.pos) {
                        if t.kind == TokenKind::Colon {
                            self.pos += 1;
                            if let Some(kind) = self.tokens.get(self.pos) {
                                tp = match kind {
                                    Token { kind: TokenKind::Int8, .. } => IntType::Int8,
                                    Token { kind: TokenKind::Uint8, .. } => IntType::Uint8,
                                    Token { kind: TokenKind::Int16, .. } => IntType::Int16,
                                    Token { kind: TokenKind::Uint16, .. } => IntType::Uint16,
                                    Token { kind: TokenKind::Int32, .. } => IntType::Int32,
                                    Token { kind: TokenKind::Uint32, .. } => IntType::Uint32,
                                    Token { kind: TokenKind::Int64, .. } => IntType::Int64,
                                    Token { kind: TokenKind::Uint64, .. } => IntType::Uint64,
                                    _ => return Err(format!("{} Type Not Exist Error AT Token {}", kind.span, self.pos))
                                };
                            }
                            self.pos += 1;
                            if let Some(e) = self.tokens.get(self.pos) {
                                if e.kind == TokenKind::Equal {
                                    expr = self.parse_expr(Types::Int(tp))?;
                                } else {
                                    return Err(format!("Expected = but found {} Error AT Token {}", e.span, self.pos))
                                }
                            }
                            let _ = self.consume_semi()?;
                        } else {
                            return Err(format!("Expected : AT Token: {} But Found {}", self.pos, t.span))
                        }
                    } else {
                        return Err(format!("Put Some Type Annotation after variable identifier AT Token: {}", id.span))
                    }
                }
                _ => return Err(format!("Expected Name After let Keyword AT Token: {} but Found {}", self.pos, id.span))
            }
        }
        let number = match expr.kind {
            NodeKind::NumberLit(n) => n,
            _ => 0
        };
        Ok(Node::new_variable(Variable::new(name, tp, number, false)))
    }

    pub fn parse_fn_type(&mut self) -> ParseResult<Node> {
        self.pos += 1;
        if let Some(ar) = self.tokens.get(self.pos) {
            match ar {
                Token { kind: TokenKind::Arrow, .. } => {
                    self.pos += 1;
                    let ret = if let Some(a) = self.tokens.get(self.pos) {
                        let typ = match a {
                            Token { kind: TokenKind::Int8, .. } => Ok(Node { kind: NodeKind::Annotation, typ: Types::Int(IntType::Int8) }),
                            Token { kind: TokenKind::Uint8, .. } => Ok(Node { kind: NodeKind::Annotation, typ: Types::Int(IntType::Uint8) }),
                            Token { kind: TokenKind::Int16, .. } => Ok(Node { kind: NodeKind::Annotation, typ: Types::Int(IntType::Int16) }),
                            Token { kind: TokenKind::Uint16, .. } => Ok(Node { kind: NodeKind::Annotation, typ: Types::Int(IntType::Uint16) }),
                            Token { kind: TokenKind::Int32, .. } => Ok(Node { kind: NodeKind::Annotation, typ: Types::Int(IntType::Int32) }),
                            Token { kind: TokenKind::Uint32, .. } => Ok(Node { kind: NodeKind::Annotation, typ: Types::Int(IntType::Uint32) }),
                            Token { kind: TokenKind::Int64, .. } => Ok(Node { kind: NodeKind::Annotation, typ: Types::Int(IntType::Int64) }),
                            Token { kind: TokenKind::Uint64, .. } => Ok(Node { kind: NodeKind::Annotation, typ: Types::Int(IntType::Uint64) }),
                            Token { kind: TokenKind::Uint0, .. } => Ok(Node { kind: NodeKind::Annotation, typ: Types::Int(IntType::Uint0) }),
                            _ => return Err(format!("Unexpected Type Token {}", a.span))
                        };
                        typ
                    } else {Err("".to_string())};
                    ret
                }
                _ => return Err(format!("Unexpected Token {}", ar.span))
            }
        } else {Err("".to_string())}
    }

    pub fn parse_scope(&mut self, typ: Types) -> ParseResult<Node> {
        self.pos += 1;
        let mut statements = Vec::new();
        match self.tokens.get(self.pos) {
            Some(x) if x.kind == TokenKind::Lbrace => {
                self.pos += 1;
                while let Some(stat) = self.tokens.get(self.pos) {
                    if stat.kind == TokenKind::Rbrace {break}
                    match stat.kind {
                        TokenKind::Let => {
                            statements.push(self.parse_variable()?);
                            self.pos += 1;
                        }
                        TokenKind::Return => {
                            statements.push(self.parse_return(typ)?);
                            self.pos += 1;
                        },
                        _ => return Err(format!("Unexpected Token {:?} AT Token: {}", stat.span, self.pos))
                    }
                }
            }
            _ => return Err("Scope is empty".to_string())
        };
        Ok(Node::new_scope(statements))
    }

    pub fn parse_expr(&mut self, typ: Types) -> ParseResult<Node> {
        self.pos += 1;
        if let Some(x) = self.tokens.get(self.pos) {
            match x {
                Token { kind: TokenKind::Int, .. } => {
                    match x.span.parse() {
                        Ok(n) => Ok(Node::new_int(n, match typ {
                            Types::Int(i) => i,
                            _ => todo!()
                        })),
                        Err(_) => Err(format!("{}", "Missing Expr".to_string()))
                    }
                },
                _ => Err(format!("Unexpected Token {}", x.span))
            }
        } else {
            Err(format!("{}", "Missing Token".to_string()))
        }
    }

    pub fn consume_semi(&mut self) -> Result<(), String> {
        self.pos += 1;
        match self.tokens.get(self.pos) {
            Some(cons) if cons.kind != TokenKind::Semi => Err(format!("Expected ; but found {}", cons.span)),
            None => Err("Expected ; but there is nothing".to_string()),
            _ => Ok(())
        }
    }
}
