
use crate::tokenizer::token::Token;

use mopa;

pub trait Node: mopa::Any {
  fn accept(&mut self, visitor: &mut NodeVisitor);
}
mopafy!(Node);  // For dynamic typing


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
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_binop(self);
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
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_integer(self);
  }
}

// UnaryNode

pub struct UnaryNode {
  pub operator: Token,
  pub value: Box<Node>,
}

impl UnaryNode { 
  pub fn new(operator: Token, value: Box<Node>) -> Self {
    UnaryNode {
      operator,
      value,
    } 
  }
}

impl Node for UnaryNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_unary(self);
  }
}

// NodeVisitor

pub trait NodeVisitor {
  fn visit(&mut self, node: &Box<Node>) -> i64 {
    if node.is::<NumNode>() {
      self.visit_integer(node.downcast_ref::<NumNode>().unwrap())
		}
    else if node.is::<BinOpNode>() {
      self.visit_binop(node.downcast_ref::<BinOpNode>().unwrap())
    } 
    else if node.is::<UnaryNode>() {
      self.visit_unary(node.downcast_ref::<UnaryNode>().unwrap())
    }
    else {
      panic!("Unknown node found");
    }
  }

  fn visit_integer(&mut self, node: &NumNode) -> i64;

  fn visit_binop(&mut self, node: &BinOpNode) -> i64;

  fn visit_unary(&mut self, node: &UnaryNode) -> i64;

}


