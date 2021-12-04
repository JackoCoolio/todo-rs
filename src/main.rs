use std::env;

extern crate todo;

use todo::parser;

fn main() {
  parser::parse_command("/home/jtwam/.todo", &mut env::args());
}
