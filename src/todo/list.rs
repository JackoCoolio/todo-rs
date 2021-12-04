use chrono::{self, DateTime, NaiveDateTime, Utc};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Clone, Copy)]
pub enum Status {
  Incomplete,
  Complete,
}

pub struct Task {
  created: chrono::DateTime<chrono::Utc>,
  status: Status,
  pub text: String,
}

impl Task {
  pub fn new(text: String) -> Task {
    Task {
      created: chrono::Utc::now(),
      status: Status::Incomplete,
      text,
    }
  }

  pub fn to_string(&self) -> String {
    let mut out = String::new();
    out += match self.status {
      Status::Complete => "[x]",
      Status::Incomplete => "[ ]",
    };

    out += " ";

    out += self.text.as_str();

    return out;
  }

  pub fn is_complete(&self) -> bool {
    match self.status {
      Status::Complete => true,
      _ => false,
    }
  }

  pub fn toggle(&mut self) -> () {
    self.status = match self.status {
      Status::Complete => Status::Incomplete,
      Status::Incomplete => Status::Complete,
    };
  }

  pub fn clone(&self) -> Task {
    Task {
      created: self.created,
      status: self.status,
      text: self.text.clone(),
    }
  }
}

pub struct List {
  pub file: File,
  pub tasks: Vec<Task>,
}

impl List {
  pub fn load_from_file(filepath: &str) -> List {
    let mut file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(filepath)
      .expect("Couldn't load todo file!");

    let mut buffer = String::new();
    file
      .read_to_string(&mut buffer)
      .expect("Couldn't read todo file!");

    let tasks = List::parse_data(&buffer).expect("Error parsing .todo file!");

    List { file, tasks }
  }

  #[allow(dead_code)]
  fn parse_data(data: &String) -> Option<Vec<Task>> {
    let mut tasks: Vec<Task> = vec![];

    for line in data.lines() {
      let mut tokens = line.trim_matches(char::from(0)).split(" ").into_iter();

      // created
      let timestamp_str = tokens.next()?;

      let timestamp: i64 = match timestamp_str.trim().parse() {
        Ok(x) => x,
        Err(e) => {
          eprintln!("{}", e);
          0
        } // return None from parse_data
      };

      let naive_date = NaiveDateTime::from_timestamp(timestamp, 0);

      // status
      let status_str = tokens.next()?;

      // text
      let words: Vec<&str> = tokens.collect();

      tasks.push(Task {
        created: DateTime::from_utc(naive_date, Utc),
        status: match status_str {
          "0" => Status::Incomplete,
          "1" => Status::Complete,
          _ => return None,
        },
        text: words.join(" "),
      })
    }

    Some(tasks)
  }

  pub fn to_string(&self) -> String {
    let mut lines: Vec<String> = vec![];

    for task in &self.tasks {
      lines.push(format!(
        "{} {} {}",
        task.created.timestamp(),
        match task.status {
          Status::Complete => "1",
          Status::Incomplete => "0",
        },
        task.text
      ));
    }

    lines.join("\n")
  }

  pub fn save(&mut self) -> std::io::Result<()> {
    self.file.set_len(0)?;
    self.file.write_all(self.to_string().as_bytes())?;

    Ok(())
  }
}
