#[macro_use]
extern crate mopa;

mod tokenizer;
mod parser;
mod interpreter;

use interpreter::Interpreter;
//use parser::Parser;
use std::fs;
use std::env;
use std::fs::File;


//use interpreter::Interpreter;

fn read_from_file(filename: &str) -> std::io::Result<String> {
  let file = File::open(filename)?;
  let mut contents = String::new();
  contents = fs::read_to_string(filename).expect("Unable to read");
  Ok(contents)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let source = read_from_file(&filename);

  println!("Processing file: {}", filename);

  let mut interpreter = Interpreter::new(source.unwrap());
  let r = interpreter.interpret();
  println!("Result: {}", r);
}


