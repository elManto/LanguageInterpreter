//#[path = "../tokenizer/mod.rs"]
//mod tokenizer;
//use tokenizer::token::Token;
//use tokenizer::token::Token::*;
//use tokenizer::Lexer;

//use tokenizer::token::Token;
//use tokenizer::token::Token::*;

use crate::tokenizer::token::Token;
use crate::tokenizer::token::Token::*;
use crate::tokenizer;
use crate::tokenizer::Lexer;


#[path = "../ast/my_ast.rs"]
mod my_ast;
use my_ast::*;

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


  fn term(&mut self) -> Box<Node> {
    //let mut result: i64 = self.factor(); 
    let mut node = self.factor();
    let mut token: tokenizer::token::Token = self.get_current_token();
    //self.eat(&token);
    while token == Multiply || token == IntegerDivision {
      match token {
        Multiply => {
          self.eat(&token);
          //result =  result * self.factor();
          node = Box::new(BinOpNode::new(node, self.factor(), token));
        }
        IntegerDivision => {
          self.eat(&token);
          //result = result / self.factor();
          node = Box::new(BinOpNode::new(node, self.factor(), token));
        }
        _ => panic!("Not a number"),
      }
      token = self.get_current_token();
    }
    node
    
  }

  fn factor(&mut self) -> Box<Node> {

    let mut current_token = self.get_current_token();

    match current_token {
      //Plus | Minus => {
      //  self.eat(&current_token);
      //  
      //}
      IntegerConst(value) => {
        current_token = self.get_current_token();
        self.eat(&current_token);
        //value.parse::<i64>().unwrap()
        Box::new(NumNode::new(value.parse::<i64>().unwrap()))
      }
      LParen => {
        self.eat(&LParen);
        let node = self.expr();
        self.eat(&RParen);
        return node;
      }
        
      //RealConst(value) => {
      //  current_token = self.get_current_token();
      //  self.eat(&current_token);
      //  value
      //}
      _ => panic!("Unhandled case in `factor()` function"),
    }
  }

  pub fn expr(&mut self) -> Box<Node> {
    let mut token_to_pick: tokenizer::token::Token = self.get_current_token();
    //let mut result: i64 = self.term();
     let mut node = self.term();
    token_to_pick = self.get_current_token();
    while token_to_pick == Plus || token_to_pick == Minus {
      match token_to_pick {
        Plus => {
          self.eat(&token_to_pick);
          node = Box::new(BinOpNode::new(node, self.term(), token_to_pick));
          //result = result + self.term()
        },
        Minus => {
          self.eat(&token_to_pick);
          node = Box::new(BinOpNode::new(node, self.term(), token_to_pick));
          //result = result - self.term()
        },
        
        _ => println!("unimplemented operator"),
      }
      
      token_to_pick = self.get_current_token();
    }
    return node;
  }


  

}
