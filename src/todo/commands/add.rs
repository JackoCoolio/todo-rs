use std::env::Args;

use crate::list::{List, Task};

use crate::commands;

pub fn run(list: &mut List, args: &mut Args) -> () {
  let words: Vec<String> = args.collect();

  list.tasks.push(Task::new(words.join(" ")));

  list
    .save()
    .expect("Something went wrong while saving .todo!");

  commands::list::print_pretty(list);
}
