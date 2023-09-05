use std::io::{stdout, self};
use crate::file::{self, get_content};
use clap::ArgMatches;
use crossterm::{execute, style::{SetForegroundColor, Color, ResetColor}};
use regex::Regex;
use crossterm::style::Print;

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

    pub fn run(&mut self, matches: &ArgMatches) -> io::Result<()>{
        if let Some(_) = matches.subcommand_matches("init") {
            file::init();
        } else if let Some(_) = matches.subcommand_matches("list") {
            let content = get_content();
            self.parse_todo(content);
            // println!("{:#?}", self.todos);
            self.print_list()?;
        }

        Ok(())
    }

    fn parse_todo(&mut self, content: String) {
      let lines: Vec<&str> = content.split("\n").filter(|&x| !x.is_empty()).collect();
      let mut todos: Vec<Todo> = Vec::new();

      for line in lines {
        let elems: Vec<&str> = line.split(" ").collect();
        let mut todo_ins = Todo::new();
        let mut content = Vec::new();

        for elem in elems {
          if App::is_done(elem) {
            todo_ins.complete = true;
          } else if App::is_date(elem) {
            if let Some(_) = todo_ins.create_at {
              todo_ins.done_at = Some(String::from(elem));
            } else {
              todo_ins.create_at = Some(String::from(elem));
            }
          } else if App::is_project(elem) {
            todo_ins.project.push(String::from(elem));
            content.push(elem);           
          } else if App::is_context(elem) {
            todo_ins.context.push(String::from(elem));
            content.push(elem);
          } else {
            content.push(elem);
          }
        }
        todo_ins.content = content.join(" ");
        todos.push(todo_ins);
      }
      self.todos = Some(todos);

    }

    fn print_list(&self) -> io::Result<()> {
      let mut stdout = stdout();
      
      if let Some(todos) = &self.todos {
        execute!(stdout, Print("\n\n"))?;

        for (index, todo) in todos.iter().enumerate() {
          let complete = if todo.complete { "✔" } else { " " };
          execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("{:<3}", index + 1)),
            ResetColor,
            Print(format!("[{}]   ", complete)),
          )?;
          App::print_content(&todo.content)?;
        }
      }

      println!("\r\n");

      Ok(())
    }

    fn print_content(s: &str) -> io::Result<()> {
      let text: Vec<&str> = s.split(" ").filter(|&x| !x.is_empty()).collect();
      let mut stdout = io::stdout();
      for value in text {
        if App::is_context(value) {
          execute!(
            stdout,
            SetForegroundColor(Color::Red),
            Print(format!("{} ", value)),
            ResetColor
          )?;
        } else if App::is_project(value) {
          execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(format!("{} ", value)),
            ResetColor
          )?;
        } else {
          execute!(
            stdout,
            Print(format!("{} ", value)),
            ResetColor
          )?;
        }
      }
      execute!(
        stdout,
        Print("\r\n"),
        ResetColor
      )?;

      Ok(())
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

    fn is_context(s: &str) -> bool {
      let re = Regex::new(r"^@(.*?)").unwrap();
      re.is_match(s)
    }
}
