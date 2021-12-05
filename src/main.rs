use std::env;

extern crate todo;
mod file;
use todo::parser;

fn main() {
  let todo_path = file::get_todo_path();
  parser::parse_command(&todo_path, &mut env::args());
}
