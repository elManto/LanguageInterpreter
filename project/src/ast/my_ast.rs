//#[path = "../tokenizer/mod.rs"]
//mod tokenizer;
//use tokenizer::token::Token;

use crate::tokenizer::token::Token;



pub trait Node: /*mopa::Any*/ {
  fn do_stuff(&self);
}

//pub struct Node {
//  pub id: i64,
//}


// BinOpNode  

pub struct BinOpNode {
  pub left: Box<Node>,
  pub right: Box<Node>,
  pub operator: Token,
}

impl BinOpNode {
  pub fn new(left: Box<Node>, right: Box<Node>, operator: Token) -> Self {
    BinOpNode {
      left,
      right,
      operator,
    }
  }
}

impl Node for BinOpNode {
  fn do_stuff(&self) {
    println!("Inside a BinOpNode");
  }
}

// NumNode 

pub struct NumNode {
  pub value: i64,
}

impl NumNode {
  pub fn new(value: i64) -> Self {
    NumNode { value }
  }
}

impl Node for NumNode {
  fn do_stuff(&self) {
    println!("Inside a NumNode");
  }
}

// NodeVisitor

pub trait NodeVisitor {
  fn visit(&mut self, node: &Box<Node>)  {
    //if node.is::<NumNode>() {

		//}
    //else if node.is::<BinOpNode>() {

    //} else {
      panic!("Unknown node found");
    //}
  }

  fn visit_integer(&mut self, node: &NumNode);

  fn visit_binop(&mut self, node: &BinOpNode) ;

}


