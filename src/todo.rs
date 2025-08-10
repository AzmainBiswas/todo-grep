use regex::Regex;

use crate::error::Result;
use core::cmp;
use core::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Todo {
    pub file_path: PathBuf,
    pub line_number: u32,
    pub content: String,
    pub priority: u8,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}: TODO({}) {}",
            self.file_path.display(),
            self.line_number,
            self.priority,
            self.content
        )
    }
}

#[derive(Debug)]
pub struct TodoList {
    pub todos: Vec<Todo>,
    re: Regex,
}

impl TodoList {
    pub fn new(re: Regex) -> Self {
        Self {
            todos: Vec::new(),
            re: re,
        }
    }

    /// find todos from a single file
    fn find_from_file(&mut self, file_path: &Path) -> Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut line_number = 0;
        let mut lines = reader.lines().into_iter();

        while let Some(line) = lines.next() {
            let line = line?;
            line_number += 1;

            if let Some(captures) = self.re.captures(&line) {
                let todo: Todo = Todo {
                    file_path: file_path.to_path_buf(),
                    line_number: line_number,
                    priority: captures.name("priority").unwrap().as_str().len() as u8,
                    content: captures.name("body").unwrap().as_str().trim().to_string(),
                };

                self.todos.push(todo);
            }
        }

        Ok(())
    }

    /// find todos from entire list of files
    pub fn find_from_files<I, P>(&mut self, files: I)
    where 
        I: IntoIterator<Item = P>,
        P: AsRef<Path> // AsRef<Path> means can be turned into a &Path
    {
        for file in files {
            let file: &Path = file.as_ref();
            if let Err(err) = self.find_from_file(file) {
                eprintln!("WARNING: can't find todo from \"{}\": {}", file.display(), err);
            }
        }
    }

    /// sort todos based on priority.
    pub fn sort(&mut self) {
        self.todos.sort_by_key(|t1| cmp::Reverse(t1.priority));
    }

    /// print all todos
    pub fn print(&self) {
        for (_, todo) in self.todos.iter().enumerate() {
            println!("{}", todo);
        }
    }
}
