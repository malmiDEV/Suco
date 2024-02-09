use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::process::exit;
use crate::lexer::{Token, TokenKind};

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
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
    Number(i64),
    String(usize),
    Identifier(String),
    Variable(Vec<HashMap<String, IntType>>),
    Scope(Vec<Node>),
    Function(String, IntType, Box<Node>, Box<Node>),
    Return(Box<Node>)
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
            kind: NodeKind::Number(val),
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

    fn new_params(args: Vec<HashMap<String, IntType>>) -> Self {
        Self {
            kind: NodeKind::Variable(args),
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

type ParseResult<T> = Result<T, ()>;

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

    pub fn parse_func(&mut self) -> Result<Node, String> {
        let identifier = self.parse_identifier()?;
        let parameter = self.parse_params()?;
        let fn_type = self.parse_fn_type()?;
        let fn_body = self.parse_scope(&fn_type.typ)?;

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

    pub fn parse_return(&mut self, typ: &Types) -> Result<Node, String> {
        let expr = self.parse_expr(typ)?;
        let _ = self.consume_semi()?;
        Ok(Node::new_return(expr))
    }

    pub fn parse_identifier(&mut self) -> Result<Node, String> {
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

    pub fn parse_params(&mut self) -> Result<Node, String> {
        self.pos += 1;
        if let Some(p) = self.tokens.get(self.pos) {
            if p.kind != TokenKind::Lparen  {
                panic!("Expected ( but found::{}",p.span)
            }
            self.pos += 1;
        }
        let mut params = Vec::new();
        while let Some(i) = self.tokens.get(self.pos) {
            if i.kind == TokenKind::Rparen {break}
            let mut name = String::new();
            let mut typ = IntType::Uint0;
            let mut hash: HashMap<String, IntType> = HashMap::new();
            if i.kind == TokenKind::Identifier {
                name.push_str(&i.span);
                self.pos += 1;
                if let Some(a) = self.tokens.get(self.pos) {
                    if a.kind == TokenKind::Colon {
                        self.pos += 1;
                        if let Some(t) = self.tokens.get(self.pos) {
                            typ = match t.span.as_str() {
                                "i8" => IntType::Int8,
                                "u8" => IntType::Uint8,
                                "i16" => IntType::Int16,
                                "u16" => IntType::Uint16,
                                "i32" => IntType::Int32,
                                "u32" => IntType::Uint32,
                                _ => panic!("Type does not exist: {}", t.span)
                            };
                            self.pos += 1;
                        }
                    } else {
                        panic!("Expected Type After Name {} _ <- expected (:) but found {}", i.span, a.span)
                    }
                    hash.insert(name, typ);
                    params.push(hash.clone());
                    if let Some(c) = self.tokens.get(self.pos) {
                        if c.kind == TokenKind::Comma {
                           self.pos += 1;
                        } else {
                           if c.kind == TokenKind::Rparen {
                               println!("{:?}",hash);
                               continue
                           } else {
                               panic!("Expected Next Parameter Or ',' Or ')' but found {}", c.span)
                           }
                        }
                    }
                }
            } else {
                panic!("Incorrect Parameter Name {}", i.span);
            }
        }
        Ok(Node::new_params(params))
    }

    pub fn parse_fn_type(&mut self) -> Result<Node, String> {
        self.pos += 1;
        if let Some(ar) = self.tokens.get(self.pos) {
            match ar {
                Token { kind: TokenKind::Arrow, .. } => {
                    self.pos += 1;
                    let mut ret = if let Some(a) = self.tokens.get(self.pos) {
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
                            _ => Err(panic!("Unexpected Type Token {}", a.span))
                        };
                        typ
                    } else {Err("".to_string())};
                    ret
                }
                _ => Err(panic!("Unexpected Token {}", ar.span))
            }
        } else {Err("".to_string())}
    }

    pub fn parse_scope(&mut self, typ: &Types) -> Result<Node, String> {
        self.pos += 1;
        let mut statements = Vec::new();
        match self.tokens.get(self.pos) {
            Some(x) if x.kind == TokenKind::Lbrace => {
                self.pos += 1;
                let r = if let Some(b) = self.tokens.get(self.pos) {
                    match b.kind {
                        TokenKind::Return => statements.push(self.parse_return(typ)?),
                        _ => panic!("Unexpected Token {:?}", b.span)
                    }
                } else {
                    panic!("There should be a statement")
                };
                r
            }
            _ => panic!("Scope is empty")
        };
        Ok(Node::new_scope(statements))
    }

    pub fn parse_expr(&mut self, typ: &Types) -> Result<Node, String> {
        self.pos += 1;
        if let Some(x) = self.tokens.get(self.pos) {
            match x {
                Token { kind: TokenKind::Int, .. } => {
                    match x.span.parse() {
                        Ok(n) => Ok(Node::new_int(n, match typ {
                            Types::Int(i) => i.clone(),
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
            Some(cons) if cons.kind != TokenKind::Semi => Err(panic!("Expected ; but found::{}", cons.span)),
            None => Err(panic!("Expected ; but there is nothing")),
            _ => Ok(())
        }
    }
}