/*
* todo-grep: grep for todos in a project
* Author: Azmain Biswas
*/

mod config;
mod error;
mod file_system;
mod todo;

use config::Config;
use error::Result;
use regex::Regex;
use std::path::Path;

use todo::TodoList;

fn main() -> Result<()> {
    let cfg = Config::new();
    let path = Path::new("./testdir/test.c");
    let files = crate::file_system::list_all_files(path, &cfg)?;

    let regex_exp: &str = r"(.*)(?i)(TODO)(?<priority>O*)(:| |-)(?<body>.*)";
    let re = Regex::new(regex_exp)?;

    let mut todos = TodoList::new(re);
    todos.find_from_files(files.into_iter());
    todos.sort();
    todos.print();

    Ok(())
}
