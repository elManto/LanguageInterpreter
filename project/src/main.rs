
//mod lexer;
#[path = "./lexer.rs"]
//use self::lexer;
mod lexer;

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
  let mut lexer = lexer::Lexer::new(source.unwrap().as_str());
  let token = lexer.get_next_token();
  println!("Got token {}", token.unwrap());

//  let mut interpreter = Interpreter::new(source.as_str());
//  match interpreter.interpret() {
//    Ok(Nil) => println!("Success!"),
//    Ok(value) => println!("Program terminated with value: {:?}", value),
//    Err(msg) => println!("!!!Error!!!: {}", msg),
//  }

}


