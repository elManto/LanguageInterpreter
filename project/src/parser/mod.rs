#[path = "../tokenizer/mod.rs"]
mod tokenizer;
use tokenizer::token::Token;
use tokenizer::token::Token::*;
use tokenizer::Lexer;

//#[path = "./token.rs"]
//mod token;

pub struct Parser {
  tokenizer: Lexer,
  token: Option<Token>,
}

impl Parser {
  pub fn new(text: &str) -> Self {
    let mut tokenizer = Lexer::new(text);
    let token = tokenizer.get_next_token();

    Parser {
      tokenizer,
      token,
    }

  }

  fn eat(&mut self , typ: &tokenizer::token::Token) {
    let current_token = self.get_current_token();

    if current_token == *typ {
      self.token = self.tokenizer.get_next_token();
    } else {
      panic!("Unexpected token!");
    }
  }

  fn get_current_token(&self) -> tokenizer::token::Token {
    return self.token.clone().unwrap();
  }


  fn term(&mut self) -> i64 {
    let token: tokenizer::token::Token = self.get_current_token();
    self.eat(&token);
    match token {
      IntegerConst(value) => value.parse::<i64>().unwrap(),
      _ => panic!("Not a number"),
    }
  }

  pub fn expr(&mut self) -> i64 {
    let mut token_to_pick: tokenizer::token::Token = self.get_current_token();
    let mut result: i64 = self.term();
    token_to_pick = self.get_current_token();
    while token_to_pick == tokenizer::token::Token::Plus || token_to_pick == tokenizer::token::Token::Minus {
      match token_to_pick {
        tokenizer::token::Token::Plus => {
          self.eat(&token_to_pick);
          result = result + self.term()
        },
        tokenizer::token::Token::Minus => {
          self.eat(&token_to_pick);
          result = result - self.term()
        },
        _ => println!("unimplemented operator"),
      }
      
      token_to_pick = self.get_current_token();
    }
    return result;
  }

  pub fn evaluate(&mut self) {
    let left: tokenizer::token::Token = self.token.as_ref().unwrap().clone();
    self.eat(&left);
    let op: tokenizer::token::Token = self.token.as_ref().unwrap().clone();
    self.eat(&op);

    let right: tokenizer::token::Token = self.token.as_ref().unwrap().clone();
    self.eat(&right);
    println!("got operator {}", op);
    println!("got operator {}", left);
    println!("got operator {}", right);
    
  }

  

}
