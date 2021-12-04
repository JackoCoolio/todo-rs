use std::env::Args;

use crate::list::List;

pub fn print_pretty(list: &List) -> () {
  for task in &list.tasks {
    println!("{}", task.to_string());
  }
}

pub fn run(list: &List, _: &mut Args) -> () {
  print_pretty(list);
}
