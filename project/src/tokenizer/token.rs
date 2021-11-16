use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
  IntegerConst(String),
  RealConst(String),
  Plus,
  Minus,
  Multiply,
  IntegerDivision,
  RealDivision,
  LParen,
  RParen,
  EOF,
}

//impl Copy for Token {
//  fn copy(&self) -> Token {
//    match self {
//     Token::IntegerConst(value) => value.clone(),
//     Token::RealConst(value) => value.clone(),
//     _ => 
//  
//    }
//  }
//}

impl<'a> fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let output = match self {
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



//pub struct Token {
//  typ: String,
//  value: String
//}
