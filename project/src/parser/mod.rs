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
pub mod my_ast;
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
      panic!("Unexpected token {}!", current_token);
    }
  }

  fn get_current_token(&self) -> tokenizer::token::Token {
    return self.token.clone().unwrap();
  }

  fn program(&mut self) -> Box<Node> {
    //self.eat(&Program);
    //let var_node = self.variable();
    //let node = self.compound_statement();
    //let token = self.get_current_token();
    //self.eat(&token);
    //node
    self.eat(&Program);
    let VarNode { identifier } = *self.variable();
    self.eat(&Semi);
    let block = self.block();
    let node = ProgramNode::new(identifier, block);
    self.eat(&Dot);
    Box::new(node)
  }

  fn block(&mut self) -> Box<Node> {
    let declarations = self.declarations();
    let compound = self.compound_statement();
    Box::new(BlockNode::new(declarations, compound))
  }

  fn declarations(&mut self) -> Vec<Box<Node>> {
    // declarations : Var (variable_declaration Semi)+ (procedure_declaration)*
    //              | (procedure_declaration)*
    //              | empty
    let mut declarations: Vec<Box<Node>> = vec![];
    if self.get_current_token() == Var {
      self.eat(&Var);
      let mut current_token = self.get_current_token();
      while let Id(_) = current_token {
        declarations.extend(self.variable_declaration());
        current_token = self.get_current_token();
        self.eat(&current_token);
        current_token = self.get_current_token();
      }
    }
    declarations
  }

  fn variable_declaration(&mut self) -> Vec<Box<Node>> {
    // variable_declaration : Id (Comma Id)* Colon type_spec
    let mut var_nodes: Vec<VarNode> = Vec::new();
    let mut identifier = self.get_current_token();
    self.eat(&identifier);

    var_nodes.push(VarNode::new(identifier));
    while self.get_current_token() == Comma {
      self.eat(&Comma);
      identifier = self.get_current_token();
      self.eat(&identifier);
      var_nodes.push(VarNode::new(identifier));
    }

    self.eat(&Colon);

    let type_node = self.type_node();
    let mut var_declarations: Vec<Box<Node>> = vec![];
    for node in var_nodes {
      let declaration = VarDeclNode::new(node, type_node.downcast_ref::<TypeNode>().unwrap().clone());
      var_declarations.push(Box::new(declaration));
    }
    var_declarations
  }

  fn type_node(&mut self) -> Box<Node> {
    let token = self.get_current_token();
    match token {
      Integer => {
        self.eat(&Integer);
        Box::new(TypeNode::new(token.clone()))
      }
      Real => {
        self.eat(&Real);
        Box::new(TypeNode::new(token.clone()))
      }
      _ => panic!("Unexpected/uncorrect type in var declaration")
    }
  }

  fn compound_statement(&mut self) -> Box<Node> {
    // compound_statement : Begin statement_list End
    let mut token = self.get_current_token();
    self.eat(&token); // BEGIN
    let nodes = self.statement_list();
    token = self.get_current_token();
    self.eat(&token); // END
    Box::new(CompoundNode::new(nodes))
  }  

  fn statement_list(&mut self) -> Vec<Box<Node>> {
    //     statement_list : statement | statement SEMI statement_list 
    let node = self.statement();
    let mut results = vec![node];

    while self.get_current_token() == Semi {
      self.eat(&Semi);
      results.append(&mut vec![self.statement()]);

      if let Id(_) = self.get_current_token() {
        panic!(
          "Invalid token in statement list: {}",
          self.get_current_token()
        )
      }
    }
    results
  }

  fn statement(&mut self) -> Box<Node> {
    // statement : compound_statement
    //           | assign_statement
    //           | empty
    match self.get_current_token() {
      Begin => self.compound_statement(),
      Id(_) => self.assignment_statement(),
      _ => self.empty(),
    }
  }
  

  fn assignment_statement(&mut self) -> Box<Node> {
    // assignment_statement : variable Assign expr
    let left = self.variable();
    let token = self.get_current_token();
    self.eat(&token);
    let right = self.expr();
    let node = AssignNode::new(left, right, token);
    Box::new(node)
  }

  fn variable(&mut self) -> Box<VarNode> {
    // variable : Id
    let token = self.get_current_token();
    if let Id(_) = token {
      self.eat(&token);
      let node = VarNode::new(token);
      Box::new(node)
    } else {
      panic!("Invalid variable");
    }
  }

  fn empty(&self) -> Box<Node> {
    Box::new(NoOpNode {})
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
        RealDivision => {
          self.eat(&token);
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
      Plus | Minus => {
        self.eat(&current_token);
        Box::new(UnaryNode::new(current_token, self.factor()))
      }
      IntegerConst(value) => {
        current_token = self.get_current_token();
        self.eat(&current_token);
        //value.parse::<i64>().unwrap()
        Box::new(NumNode::new(value.parse::<i64>().unwrap()))
      }
      RealConst(value) => {
        current_token = self.get_current_token();
        self.eat(&current_token);
        Box::new(FloatNode::new(value.parse::<f64>().unwrap()))
      }
      LParen => {
        self.eat(&LParen);
        let node = self.expr();
        self.eat(&RParen);
        return node;
      }
      Id(value) => {
        self.variable()
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

  pub fn parse(&mut self) -> Box<Node> {
    let node = self.program();
    let current_token = self.get_current_token();
    if current_token != EOF {
      panic!("Unexpected token found at end of file: {}", current_token);
    }
    node
  }



  

}
