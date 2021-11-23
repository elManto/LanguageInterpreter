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

// FloatNode

pub struct FloatNode {
  pub value: f64,
}

impl FloatNode {
  pub fn new(value: f64) -> Self {
    FloatNode { value }
  }
}

impl Node for FloatNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_float(self);
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

// CompoundNode i.e., list of statements

pub struct CompoundNode {
  pub children: Vec<Box<Node>>,
}

impl CompoundNode {
  pub fn new(children: Vec<Box<Node>>) -> Self {
    CompoundNode { children }
  }
}

impl Node for CompoundNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_compound(self);
  }
}

// AssignNode 

pub struct AssignNode {
  pub identifier: Box<Node>,
  pub expr: Box<Node>,
  pub operator: Token,
}

impl AssignNode {
  pub fn new(identifier: Box<Node>, expr: Box<Node>, operator: Token) -> Self {
    AssignNode {
      identifier,
      expr,
      operator,
    }
  }
}

impl Node for AssignNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_assign(self);
  }
}

// VarNode

pub struct VarNode {
  pub identifier: Token,
}

impl VarNode {
  pub fn new(identifier: Token) -> Self {
    VarNode { identifier }
  }
}

impl Node for VarNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_var(self);
  }
}

// NoOpNode

pub struct NoOpNode {}

impl Node for NoOpNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_noop(self);
  }
}

// ProgramNode

pub struct ProgramNode {
  pub identifier: Token,
  pub block: Box<Node>,
}

impl ProgramNode {
  pub fn new(identifier: Token, block: Box<Node>) -> Self {
    ProgramNode { identifier, block }
  }
}

impl Node for ProgramNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_program(self);
  }
}

// BlockNode

pub struct BlockNode {
  pub declarations: Vec<Box<Node>>,
  pub compound_statement: Box<Node>,
}

impl BlockNode {
  pub fn new(declarations: Vec<Box<Node>>, compound_statement: Box<Node>) -> Self {
    BlockNode {
      declarations,
      compound_statement,
    }
  }
}

impl Node for BlockNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_block(self);
  }
}

// VarDeclNode

pub struct VarDeclNode {
  pub var_node: VarNode,
  pub type_node: TypeNode,
}

impl VarDeclNode {
  pub fn new(var_node: VarNode, type_node: TypeNode) -> Self {
    VarDeclNode {
      var_node,
      type_node,
    }
  }
}

impl Node for VarDeclNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_vardecl(self);
  }
}


// TypeNode

#[derive(Clone)]
pub struct TypeNode {
  pub token: Token,
}

impl TypeNode {
  pub fn new(token: Token) -> Self {
    TypeNode { token }
  }
}

impl Node for TypeNode {
  fn accept(&mut self, visitor: &mut NodeVisitor) {
    visitor.visit_type(self);
  }
}


// NodeVisitor

pub trait NodeVisitor {
  fn visit(&mut self, node: &Box<Node>) -> i64 {
    if node.is::<NumNode>() {
      self.visit_integer(node.downcast_ref::<NumNode>().unwrap())
		}
    else if node.is::<FloatNode>() {
      self.visit_float(node.downcast_ref::<FloatNode>().unwrap())
    }
    else if node.is::<BinOpNode>() {
      self.visit_binop(node.downcast_ref::<BinOpNode>().unwrap())
    } 
    else if node.is::<UnaryNode>() {
      self.visit_unary(node.downcast_ref::<UnaryNode>().unwrap())
    }
    else if node.is::<ProgramNode>() {
      self.visit_program(node.downcast_ref::<ProgramNode>().unwrap())
    }
    else if node.is::<NoOpNode>() {
      self.visit_noop(node.downcast_ref::<NoOpNode>().unwrap())
    }
    else if node.is::<VarNode>() {
      self.visit_var(node.downcast_ref::<VarNode>().unwrap())
    }
    else if node.is::<AssignNode>() {
      self.visit_assign(node.downcast_ref::<AssignNode>().unwrap())
    }
    else if node.is::<CompoundNode>() {
      self.visit_compound(node.downcast_ref::<CompoundNode>().unwrap())
    }
    else if node.is::<BlockNode>() {
      self.visit_block(node.downcast_ref::<BlockNode>().unwrap())
    }
    else if node.is::<VarDeclNode>() {
      self.visit_vardecl(node.downcast_ref::<VarDeclNode>().unwrap())
    }
    else if node.is::<TypeNode>() {
      self.visit_type(node.downcast_ref::<TypeNode>().unwrap())
    }
    else {
      panic!("Unknown node found");
    }
  }

  fn visit_integer(&mut self, node: &NumNode) -> i64;
  fn visit_float(&mut self, node: &FloatNode) -> i64;
  fn visit_binop(&mut self, node: &BinOpNode) -> i64;
  fn visit_unary(&mut self, node: &UnaryNode) -> i64;
  fn visit_program(&mut self, node: &ProgramNode) -> i64;
  fn visit_noop(&mut self, node: &NoOpNode) -> i64;
  fn visit_var(&mut self, node: &VarNode) -> i64;
  fn visit_assign(&mut self, node: &AssignNode) -> i64;
  fn visit_compound(&mut self, node: &CompoundNode) -> i64;
  fn visit_block(&mut self, node: &BlockNode) -> i64;
  fn visit_vardecl(&mut self, node: &VarDeclNode) -> i64;
  fn visit_type(&mut self, node: &TypeNode) -> i64;
}


