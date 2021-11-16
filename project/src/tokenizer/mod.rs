//pub mod token;
//#[path = "./token.rs"]
pub mod token;

use self::token::Token;
use self::token::Token::*;



pub struct Lexer {
  text: String,
  pos: usize,
  current: Option<char>,
}

impl Lexer {
  pub fn new(text: &str) -> Self {
    let characters: Vec<char> = text.chars().collect();
    return Lexer {
      text: text.to_string(),
      pos: 0,
      current: Some(characters[0]),
    }
  }

  fn skip_whitespace(&mut self) {
    while self.current != None && self.current.unwrap().is_whitespace() {
      self.advance()
    }
  }

  fn advance(&mut self) {
    self.pos += 1;
    if self.pos > self.text.len() - 1 {
      self.current = None;
    }
    else {
     let characters: Vec<char> = self.text.chars().collect();
     self.current = Some(characters[self.pos]);
    }

  }

  fn number(&mut self) -> token::Token {
    let mut digits = String::new();
    while self.current != None && self.current.unwrap().is_digit(10) {
      digits.push(self.current.unwrap());
      self.advance();

      if self.current == Some('.') {
        digits.push('.');
        self.advance();

        while self.current != None && self.current.unwrap().is_digit(10) {
          digits.push(self.current.unwrap());
          self.advance();
        }
        return token::Token::RealConst(digits);
      }
    }
    token::Token::IntegerConst(digits)
  }

  
  pub fn get_next_token(&mut self) -> Option<token::Token> {

    while self.current != None {
      let next_token: token::Token =  match self.current.unwrap() {
        char if char.is_whitespace() => {
          self.skip_whitespace();
          continue;
        }
        char if char.is_digit(10) => {
          let token = self.number();
          token
        }
        '+' => {
          self.advance();
          token::Token::Plus
        }
        '-' => {
          self.advance();
          token::Token::Minus
        }
        '*' => {
          self.advance();
          token::Token::Multiply
        }
        '/' => {
          self.advance();
          token::Token::IntegerDivision
        }
        '(' => {
          self.advance();
          token::Token::LParen
        }
        ')' => {
          self.advance();
          token::Token::RParen
        }
        _ => {
          panic!("Unknown token: {}", self.current.unwrap())
        }
      };

      return Some(next_token);

    }
    return Some(token::Token::EOF);
  }
  
}
