#[cfg(target_os = "windows")]
use std::env;

#[cfg(target_os = "windows")]
pub fn get_todo_path() -> String {
  let mut dir = env::current_exe().expect("Couldn't get todo path!");
  dir.pop();
  dir.push("todo");
  let dir_str = dir.to_string_lossy();
  dir_str.to_string()
}

#[cfg(target_os = "linux")]
pub fn get_todo_path() -> String {
  let mut dir = home::home_dir().expect("Couldn't get todo path!");
  dir.push(".todo");
  let dir_str = dir.to_string_lossy();
  dir_str.to_string()
}
