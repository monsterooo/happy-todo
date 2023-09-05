use crate::file::{self, get_content};
use clap::ArgMatches;
use chrono::{NaiveDate};
use regex::Regex;

#[derive(Default, Debug)]
pub struct Todo {
    complete: bool,
    content: String,
    create_at: Option<String>,
    done_at: Option<String>,
    project: Vec<String>,
    context: Vec<String>,
}


impl Todo {
  fn new() -> Self {
    Todo { ..Default::default() }
  }
}

pub struct App {
    todos: Option<Vec<Todo>>,
}

impl App {
    pub fn new() -> Self {
      App { todos: None }
    }

    pub fn run(&mut self, matches: &ArgMatches) {
        if let Some(_) = matches.subcommand_matches("init") {
            file::init();
        } else if let Some(_) = matches.subcommand_matches("list") {
            let content = get_content();
            println!("{}", content);
            self.parse_todo(content);
        }
    }

    fn parse_todo(&mut self, content: String) {
      let lines: Vec<&str> = content.split("\n").collect();
      let mut todos: Vec<Todo> = Vec::new();

      for line in lines {
        let elems: Vec<&str> = line.split(" ").collect();
        let mut todoIns = Todo::new();
        // println!("{:?}", elems);
        for elem in elems {
          if elem.len() < 1 {
            continue;
          }
          if App::is_done(elem) {
            todoIns.complete = true;
          } else if App::is_date(elem) {
            if let Some(_) = todoIns.create_at {
              todoIns.done_at = Some(String::from(elem));
            } else {
              todoIns.create_at = Some(String::from(elem));
            }
          } else if App::is_project(elem) {
            println!("project: {}", elem);
            
          }
        }
        todos.push(todoIns);
      }
      self.todos = Some(todos);

    }

    fn is_done(s: &str) -> bool {
      s == "x"
    }

    fn is_date(s: &str) -> bool {
      let re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();
      re.is_match(s)
    }

    fn is_project(s: &str) -> bool {
      let re = Regex::new(r"^\+(.*?)").unwrap();
      re.is_match(s)
    }
}
