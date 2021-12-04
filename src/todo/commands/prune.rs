use std::env::Args;

use crate::commands;
use crate::list::List;

pub fn run(list: &mut List, _: &mut Args) -> () {
  let tasks = list
    .tasks
    .iter()
    .filter(|task| !task.is_complete())
    .map(|t| t.clone())
    .collect::<Vec<_>>();

  // overwrite old tasks
  list.tasks = tasks;

  list
    .save()
    .expect("Something went wrong while saving .todo!");

  commands::list::print_pretty(list);
}
