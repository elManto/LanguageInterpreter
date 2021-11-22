//#[path = "../ast/my_ast.rs"]
//mod my_ast;
//use my_ast::{Node, NodeVisitor, BinOpNode, NumNode};

//use crate::ast::my_ast::{Node, NodeVisitor, BinOpNode, NumNode};

use crate::parser::Parser;
//use crate::parser::my_ast::{Node, NodeVisitor, BinOpNode, NumNode, UnaryNode};
use crate::parser::my_ast::*;
use crate::tokenizer::token::Token::{Id, Multiply, Plus, IntegerDivision, Minus};

use std::collections::HashMap;



pub struct Interpreter {
  //pub root_node: Box<Node>,
  pub text: String,
  pub symbol_table: HashMap<String, String>,
}

impl Interpreter {
  pub fn new(text: String) -> Self {
   Interpreter { 
      text,
      symbol_table: HashMap::new(),
    }
  }
  pub fn interpret(&mut self) -> i64 {
    let mut parser = Parser::new(self.text.as_str());
    //let root_node = parser.expr();
    let root_node = parser.parse();

    let res = self.visit(&root_node);
    // Debug 
    for (k, v) in &self.symbol_table {
      println!("Key {} -> Value {}", k, v);
    }
    res
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

      //println!("Operator -> {}", Multiply == *operator);
      if *operator == Multiply {
        println!("Multiply -> {}", operator);
        lhs * rhs
      }
      else if *operator == Plus {
        println!("Plus -> {}", operator);
        lhs + rhs
      }
      else if *operator == IntegerDivision{ 
        println!("Division -> {}", operator);
        lhs / rhs
      }
      else if *operator == Minus {
        println!("Minus -> {}", operator);
        lhs - rhs
      }
      else {
        panic!("Unknown operator found: {}", operator);
      }

    }
    
    fn visit_integer(&mut self, node: &NumNode) -> i64 {
      node.value
    }

    fn visit_unary(&mut self, node: &UnaryNode) -> i64 {
      let UnaryNode {operator, value} = node;
      match operator {
        Plus => self.visit(value),
        Minus => (-1) * self.visit(value),
        _ => panic!("Wrong Unary Operator"),
      }
    }

    fn visit_program(&mut self, node: &ProgramNode) -> i64 {
      self.visit(&node.block)
    }

    fn visit_noop(&mut self, node: &NoOpNode) -> i64 {
      -1
    }

    fn visit_var(&mut self, node: &VarNode) -> i64 {
      if let VarNode {
        identifier: Id(name),
      } = node
      {
        match self.symbol_table.get(name.as_str()) {
          Some(value) => value.parse::<i64>().unwrap(),
          None => {
            println!("Use of uninitialised varible");
            -1
          }
        }
      } else {
        panic!("Invalid identifier found")
      }
 
    }

    fn visit_assign(&mut self, node: &AssignNode) -> i64 {
      if node.identifier.is::<VarNode>() {
        let var_node: &VarNode = node.identifier.downcast_ref().unwrap();
        if let Id(name) = &var_node.identifier {
          let value = self.visit(&node.expr);
          self
            .symbol_table
            .insert(name.to_string(), value.to_string());
        }
      }
      -1
    }

    fn visit_compound(&mut self, node: &CompoundNode) -> i64 {
      for child in &node.children {
        self.visit(child); 
      }
      0
    }
    
}

