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
    let given_path = std::env::args().nth(1).unwrap_or(".".to_string());
    let cfg = Config::new();
    let path = Path::new(&given_path);
    let files = match crate::file_system::list_all_files(path, &cfg) {
        Ok(_files) => _files,
        Err(_err) => {
            eprintln!(
                "ERROR: can't do anything for \"{}\": {}",
                path.display(),
                _err
            );
            std::process::exit(10);
        }
    };

    let regex_exp: &str = r"(.*)(?i)(TODO)(?<priority>O*)(:| |-)(?<body>.*)";
    let re = match Regex::new(regex_exp) {
        Ok(_err) => _err,
        Err(_err) => {
            eprintln!("ERROR: can't compile regex: {}", _err);
            std::process::exit(3);
        }
    };

    let mut todos = TodoList::new(re);
    todos.find_from_files(files.into_iter());
    todos.sort();
    todos.print();

    Ok(())
}
