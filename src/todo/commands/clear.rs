use std::env::Args;

use crate::list::List;

pub fn run(todo: &mut List, _: &mut Args) -> () {
  todo.tasks.clear();
  todo
    .save()
    .expect("Something went wrong whlie saving .todo!");
}
