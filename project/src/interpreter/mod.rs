//#[path = "../ast/my_ast.rs"]
//mod my_ast;
//use my_ast::{Node, NodeVisitor, BinOpNode, NumNode};

//use crate::ast::my_ast::{Node, NodeVisitor, BinOpNode, NumNode};

use crate::parser::Parser;
use crate::parser::my_ast::{Node, NodeVisitor, BinOpNode, NumNode};


pub struct Interpreter {
  //pub root_node: Box<Node>,
  pub text: String,
}

impl Interpreter {
  pub fn new(text: String) -> Self {
   Interpreter { text }
  }
  pub fn interpret(&mut self) -> i64 {
    let mut parser = Parser::new(self.text.as_str());
    let root_node = parser.expr();

    self.visit(&root_node)
 
    //self.visit(&root_node);


  }
}

impl NodeVisitor for Interpreter {
    fn visit_binop(&mut self, node: &BinOpNode) -> i64 {
      let BinOpNode {
        left,
        right,
        operator,
      } = node;

      let lhs = self.visit(left);
      let rhs = self.visit(right);
      match operator {
        Plus => lhs + rhs,
        Multiply => lhs * rhs,
        Minus => lhs - rhs,
        IntegerDivision => lhs / rhs,
        _ => panic!("Unknown operator found: {}", operator),
      }
    }
    
    fn visit_integer(&mut self, node: &NumNode) -> i64 {
      node.value
    }
    
}

