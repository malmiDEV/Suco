use std::fmt::{Display, Formatter, write};
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
   None,

   Int,
   Float,
   Uint0,
   Int8,
   Uint8,
   Int16,
   Uint16,
   Int32,
   Uint32,
   Int64,
   Uint64,

   // ketwords
   Defun,
   Return,

   // separators
   Lbrace,
   Rbrace,
   Lparen,
   Rparen,
   Arrow,
   Colon,
   Comma,
   Semi,
   
   // operators
   Plus,
   Minus,

   // etc
   WhiteSpace,
   Eof,
   Identifier
}

impl Display for TokenKind {
   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      match self {
         TokenKind::None => write!(f, "None"),
         TokenKind::Int => write!(f, "Int"),
         TokenKind::Float => write!(f, "Float"),
         TokenKind::Uint0 => write!(f, "Uint0"),
         TokenKind::Int8 => write!(f, "Int8"),
         TokenKind::Uint8 => write!(f, "Uint8"),
         TokenKind::Int16 => write!(f, "Int16"),
         TokenKind::Uint16 => write!(f, "Uint16"),
         TokenKind::Int32 => write!(f, "Int32"),
         TokenKind::Uint32 => write!(f, "Uint32"),
         TokenKind::Int64 => write!(f, "Int64"),
         TokenKind::Uint64 => write!(f, "Uint64"),
         TokenKind::Defun => write!(f, "Defun"),
         TokenKind::Return => write!(f, "Return"),
         TokenKind::Lbrace => write!(f, "Lbrace"),
         TokenKind::Rbrace => write!(f, "Rbrace"),
         TokenKind::Lparen => write!(f, "Lparen"),
         TokenKind::Rparen => write!(f, "Rparen"),
         TokenKind::Arrow => write!(f, "Arrow"),
         TokenKind::Colon => write!(f, "Colon"),
         TokenKind::Comma => write!(f, "Comma"),
         TokenKind::Semi => write!(f, "Semi"),
         TokenKind::Plus => write!(f, "Plus"), 
         TokenKind::Minus => write!(f, "Minus"),
         TokenKind::WhiteSpace => write!(f, "WhiteSpace"),
         TokenKind::Eof => write!(f, "Eof"),
         TokenKind::Identifier => write!(f, "Identifier")
      }
   } 
}

#[derive(Debug)]
pub struct Token {
   pub kind: TokenKind,
   pub span: String
}

impl Token {
   fn new(kind: TokenKind, span: String) -> Self {
      Self { kind, span }
   }
}

pub struct Lexer<'a> {
   code: &'a str,
   pos: usize
}

impl<'a> Lexer<'a> {
   pub fn new(code: &'a str) -> Self {
      Self { code, pos: 0 }
   }

   pub fn tokenize(&mut self) -> Option<Token> {
      if self.pos == self.code.len() {
         self.pos += 1;
         return Some(Token::new(TokenKind::Eof, "\0".to_string()));
      }
      if let Some(c) = self.lexer_peek_code() {
         let mut kind = TokenKind::None;
         let mut span = String::new();
         if c.is_whitespace() {
            kind = TokenKind::WhiteSpace;
            self.pos += 1;
         } else if c.is_digit(10) {
            while let Some(c) = self.lexer_peek_code() {
               if c.is_digit(10) {
                  kind = TokenKind::Int;
                  self.pos += 1;
                  span.push(c);
               } else {
                  let valid = match self.lexer_peek_advance() {
                     Some(c) if c.is_digit(10) => true,
                     _ => false
                  };
                  if c == '.' && valid {
                     span.push('.');
                     while let Some(c) = self.lexer_peek_code() {
                        if !c.is_digit(10) {break}
                        self.pos += 1;
                        span.push(c);
                     }
                  } else {
                     self.pos -= 1;
                     break;
                  }
                  kind = TokenKind::Float;
                  break;
               }
            }
         } else if c.is_alphanumeric() {
            let mut buffer = String::new();
            while let Some(x) = self.lexer_peek_code() {
               if !x.is_alphanumeric() {break}
               buffer.push(x);
               self.pos += 1;
            }
            let input = buffer.as_str();
            kind = match input {
               "defun" => TokenKind::Defun,
               "return" => TokenKind::Return,
               "u0" => TokenKind::Uint0,
               "i8" => TokenKind::Int8,
               "u8" => TokenKind::Uint8,
               "i16" => TokenKind::Int16,
               "u16" => TokenKind::Uint16,
               "i32" => TokenKind::Int32,
               "u32" => TokenKind::Uint32,
               "i64" => TokenKind::Int64,
               "u64" => TokenKind::Uint64,
               _ => TokenKind::Identifier
            };
            span.push_str(buffer.as_str());
         } else if c.is_ascii_punctuation() {
            let mut advance: String = String::new();
            if let Some(p) = self.lexer_peek_code() {
               if !p.is_ascii_punctuation() { panic!("Unexpected Token") }

               kind = match p {
                  '{' => {
                     span.push(p);
                     TokenKind::Lbrace
                  },
                  '}' => {
                     span.push(p);
                     TokenKind::Rbrace
                  }
                  '(' => {
                     span.push(p);
                     TokenKind::Lparen
                  }
                  ')' => {
                     span.push(p);
                     TokenKind::Rparen
                  }
                  '+' => {
                     span.push(p);
                     TokenKind::Plus
                  }
                  '-' => {
                     if let Some(x) = self.lexer_peek_code_more() {
                        self.pos += 1;
                        let mut kind = TokenKind::None;
                        if x == '>' {
                           kind = TokenKind::Arrow;
                           span.push_str("->");
                        }
                        kind
                     } else {
                        span.push('-');
                        TokenKind::Minus
                     }
                  },
                  ':' => {
                     span.push(p);
                     TokenKind::Colon
                  }
                  ',' => {
                     span.push(p);
                     TokenKind::Comma
                  }
                  ';' => {
                     span.push(p);
                     TokenKind::Semi
                  }
                  _ => TokenKind::None,
               };
               self.pos += 1;
               // println!("{:?} {:?}", kind,span);
               // exit(-1);
            }
         } else{
            eprintln!("Token Not Exist");
            exit(-1);
         }
         // exit(-1);
         Some(Token::new(kind, span))
      } else {
         None
      }
   }

   fn lexer_peek_code(&self) -> Option<char> {
      self.code.chars().nth(self.pos)
   }

   fn lexer_peek_code_more(&self) -> Option<char> {
      self.code.chars().nth(self.pos+1)
   }

   fn lexer_peek_advance(&mut self) -> Option<char> {
      self.pos += 1;
      let c = self.lexer_peek_code();
      c
   }
}