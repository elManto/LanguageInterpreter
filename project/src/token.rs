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
  EOF,
}


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
      Token::EOF => "EOF",
    };
    write!(f, "{}", output)
  }
}



//pub struct Token {
//  typ: String,
//  value: String
//}
