use std::env::Args;

use crate::list::List;

use crate::commands;

pub fn run(list: &mut List, args: &mut Args) -> () {
  let words: Vec<String> = args.collect();

  let query = words.join(" ");

  let mut found = false;

  // weird way to remove first matching task
  let tasks = list
    .tasks
    .iter()
    .filter(|task| {
      if !found && task.text.starts_with(&query) {
        found = true;
        false
      } else {
        true
      }
    })
    .map(|t| t.clone())
    .collect::<Vec<_>>();

  // overwrite old tasks
  list.tasks = tasks;

  list
    .save()
    .expect("Something went wrong while saving .todo!");

  commands::list::print_pretty(list);
}
