# todo

A command-line todo list tracker written in Rust.

## Installation
1. Clone the repository with `git clone https://github.com/JackoCoolio/todo`
2. `cd todo`
3. Build with `cargo build`
4. Install with `cargo install --path .`

## Usage
`todo [command] [args]`
### Commands
- `list` or no command: Lists the contents of the todo list.
- `add <text>`: Adds a new item to the todo list.
- `toggle <item>`: Toggles the checkbox of the corresponding todo item.
  - Toggles the first item that starts with `<item>`.
  - Aliases: `mark`, `done`
- `clear`: Deletes all of the items.
- `prune`: Deletes all checked items.
  - Aliases: `filter`
- `delete <item>`: Deletes the corresponding todo item.
  - `<item>` functions just like it does for the `toggle` command. It matches the first item that starts with `<item>`.
  - Aliases: `remove`
