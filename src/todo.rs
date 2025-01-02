use regex::{Captures, Regex};
use std::cmp;

#[derive(Debug)]
pub struct TODO {
    file_name: String,
    line_number: usize,
    body: String,
    priority: u16,
}

impl TODO {
    fn new(file_name: String, line_number: usize, body: String, priority: u16) -> Self {
        Self {
            file_name,
            line_number,
            body,
            priority
        }
    }

    /// print todo
    fn print(&self) {
        println!("{}:{}: TODO({}) {}", self.file_name, self.line_number, self.priority, self.body);
    }
}

/// parce todo from file content
pub fn parce_todo(file_path: &str, file_content: &str, regex_exp: &str) -> Vec<TODO> {
    let mut todos: Vec<TODO> = Vec::new();
    let reg: Regex = Regex::new(regex_exp).unwrap();

    for (line_number, line) in file_content.lines().enumerate() {
        if reg.is_match(line) {
            let capture: Captures = reg.captures(line).unwrap();
            let tpriority: u16 = capture.name("priority").unwrap().len() as u16;
            let tbody: &str = capture.name("body").unwrap().as_str().trim_start();
            todos.push(TODO::new(file_path.to_string(), line_number + 1, tbody.to_string(), tpriority + 1));
        }
    }

    return todos;
}

pub fn sort_todos(todos: &mut Vec<TODO>) {
    todos.sort_by_key(|todo| cmp::Reverse(todo.priority));
}

/// print todo list
pub fn print_todo(todos: Vec<TODO>) {
    for todo in todos {
        todo.print();
    }
}
