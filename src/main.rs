mod todo;

use std::fs;
use std::io::Result;
use std::process::exit;

// read a file content to a string buffer
fn read_file_content_as_string(file_path: &str) -> Result<String> {
    let file_content: String = fs::read_to_string(file_path)?;
    Ok(file_content)
}

fn main() {
    let file_path: &str = "./test.txt";
    let todo_regex_exp: &str = r"(.*)(?i)(TODO)(?<priority>O*)(:| |-)(?<body>.*)";
    let content: String = read_file_content_as_string(file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: could not read -> {file_path}\n\t{err}");
        exit(1)
    });
    
    let mut todos = todo::parce_todo(file_path, &content, todo_regex_exp);
    todo::sort_todos(&mut todos);
    todo::print_todo(todos);
}
