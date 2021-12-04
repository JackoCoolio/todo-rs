use std::env::Args;

use crate::commands;
use crate::list::List;

pub fn run(list: &mut List, args: &mut Args) -> () {
  let words: Vec<String> = args.collect();
  let query = words.join(" ");

  for task in &mut list.tasks {
    if task.text.starts_with(&query) {
      task.toggle();

      commands::list::print_pretty(list);
      list
        .save()
        .expect("Something went wrong while saving .todo!");

      return; // only toggle the first one
    }
  }

  println!("No matching list item was found!");
}
