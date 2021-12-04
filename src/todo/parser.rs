use std::env::Args;

use crate::{commands, list::List};

pub fn parse_command(filepath: &str, args: &mut Args) {
  let command_res = args.nth(1);

  let mut list = List::load_from_file(filepath);

  match command_res {
    Some(command) => match command.as_str() {
      "add" => commands::add::run(&mut list, args),
      "clear" => commands::clear::run(&mut list, args),
      "toggle" | "mark" | "done" => commands::toggle::run(&mut list, args),
      "delete" | "remove" => commands::delete::run(&mut list, args),
      "prune" | "filter" => commands::prune::run(&mut list, args),
      _ => {
        eprintln!("Invalid command '{}'", command);
      }
    },
    None => commands::list::run(&list, args),
  };
}
