use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
  Program,
  Procedure,
  Var,
  Integer,
  Real,
  Begin,
  End,
  Dot,
  Id(String),
  Assign,
  Semi,
  IntegerConst(String),
  RealConst(String),
  Plus,
  Minus,
  Multiply,
  Div,
  IntegerDivision,
  RealDivision,
  LParen,
  RParen,
  EOF,
}


impl<'a> fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let output = match self {
      Token::Program => "PROGRAM",
      Token::Procedure => "PROCEDURE",
      Token::Var => "VAR",
      Token::Integer => "INTEGER",
      Token::Real => "REAL",
      Token::Div => "DIV",
      Token::Begin => "BEGIN",
      Token::End => "END",
      Token::Dot => ".",
      Token::Id(value) => value,
      Token::Assign => ":=",
      Token::Semi => ";",
      Token::IntegerConst(value) => value,
      Token::RealConst(value) => value,
      Token::Plus => "+",
      Token::Minus => "-",
      Token::Multiply => "*",
      Token::IntegerDivision => "DIV",
      Token::RealDivision => "/",
      Token::LParen => "(",
      Token::RParen => ")",
      Token::EOF => "EOF",
    };
    write!(f, "{}", output)
  }
}
