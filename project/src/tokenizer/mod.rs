//pub mod token;
//#[path = "./token.rs"]
pub mod token;

use self::token::Token;
use self::token::Token::*;
use std::collections::HashMap;


pub struct Lexer {
  text: String,
  pos: usize,
  current: Option<char>,
  reserved_words: HashMap<&'static str, token::Token>,
}

impl Lexer {
  pub fn new(text: &str) -> Self {
    let characters: Vec<char> = text.chars().collect();
    let mut reserved_words = HashMap::new();
    reserved_words.insert("PROGRAM", token::Token::Program);
    reserved_words.insert("PROCEDURE", token::Token::Procedure);
    reserved_words.insert("VAR", token::Token::Var);
    reserved_words.insert("INTEGER", token::Token::Integer);
    reserved_words.insert("REAL", token::Token::Real);
    reserved_words.insert("BEGIN", token::Token::Begin);
    reserved_words.insert("END", token::Token::End);
    reserved_words.insert("DIV", token::Token::IntegerDivision);
    return Lexer {
      text: text.to_string(),
      pos: 0,
      current: Some(characters[0]),
      reserved_words,
    }
  }

  fn skip_whitespace(&mut self) {
    while self.current != None && self.current.unwrap().is_whitespace() {
      self.advance()
    }
  }

  fn skip_comments(&mut self) {
    while self.current != None && self.current.unwrap() != '}' {
      self.advance();
    }
    self.advance();
  }

  pub fn next(&self) -> Option<char> {
    let pos = self.pos + 1;
    if pos > self.text.len() - 1 {
      None
    } else {
      Some(self.text.as_bytes()[pos] as char)
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

  fn id(&mut self) -> token::Token {
    let mut current_id = String::new();
    while self.current != None && self.current.unwrap().is_alphanumeric() {
      current_id.push(self.current.unwrap());
      self.advance();
    }
    let upper_current_id = current_id.to_uppercase();
    if self.reserved_words.contains_key(upper_current_id.as_str()) {
      self.reserved_words.get(upper_current_id.as_str()).unwrap().clone()
    } else {
      Id(current_id)
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
        '{' => {
          self.advance();
          self.skip_comments();
          continue;
        }
        char if char.is_digit(10) => {
          let token = self.number();
          token
        }
        char if char.is_alphanumeric() => self.id(),
        '_' if self.next().unwrap().is_alphanumeric() => {
          self.advance();
          self.id()
        }
        ':' if self.next() == Some('=') => {
          self.advance();
          self.advance();
          token::Token::Assign
        }
        ':' => {
          self.advance();
          token::Token::Colon
        }
        ',' => {
          self.advance();
          token::Token::Comma
        }
        ';' => {
          self.advance();
          token::Token::Semi
        }
        '.' => {
          self.advance();
          token::Token::Dot
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
          token::Token::RealDivision
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
