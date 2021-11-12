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

  fn eat(&mut self /*, typ: tokenizer::token::Token*/) {
    self.token = self.tokenizer.get_next_token();
    //if self.token.as_ref() == Some(&typ) {
    //  self.token = self.tokenizer.get_next_token();
    //}
    //else {
    //  panic!("The eat function found an unexptected token type");
    // }
  }

  pub fn evaluate(&mut self) {
    //let left: &mut tokenizer::token::Token = self.token.unwrap();
    //self.eat();
    //let op: &tokenizer::token::Token = self.token.as_ref().unwrap();
    //self.eat();
    //let right: &tokenizer::token::Token = self.token.as_ref().unwrap();
    //self.eat();
    //println!("got operator {}", op);
    
       


  }



}
