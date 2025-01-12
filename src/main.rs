mod todo;

use crate::todo::TODO;
use std::env::args;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::process::exit;

/// read a file content to a string buffer
fn read_file_content_as_string(file_path: &str) -> Result<String> {
    let file_content: String = fs::read_to_string(file_path)?;
    Ok(file_content)
}

/// print usage
fn print_usage(app_location: &str) {
    eprintln!("USAGE: {app_location} <Directory/file> <Option>     To See all the TODOs.");
    eprintln!("       Option: -s or --sort                         To Sort the TODOs.");
}

fn extract_all_file_from_folder(path: PathBuf, ignore_dir: Vec<&str>) -> Result<Vec<String>> {
    // TODO: Add error handeles
    let mut files: Vec<String> = Vec::new();
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.to_str().unwrap();

            if ignore_dir.iter().any(|ign| dir_name.ends_with(ign)) {
                continue;
            }

            files.append(
                &mut extract_all_file_from_folder(path, ignore_dir.clone()).unwrap_or_else(|err| {
                    eprintln!("ERROR: SomeThing Wrong: {err}");
                    exit(10)
                }),
            );
        } else if path.is_file() {
            files.push(String::from(
                path.to_str().expect("ERROR: Can't convert to string"),
            ));
        }
    }

    Ok(files)
}

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        exit(10);
    }

    let path: &str = &args[1];
    let file_type = fs::metadata(path)
        .expect(&format!("ERROR: Can't Open {path}"))
        .file_type();

    let todo_regex_exp: &str = r"(.*)(?i)(TODO)(?<priority>O*)(:| |-)(?<body>.*)";
    let mut todos: Vec<TODO> = Vec::new();

    if file_type.is_file() {
        // for file
        let file_path = path;
        let content: String = read_file_content_as_string(file_path).unwrap_or_else(|err| {
            eprintln!("ERROR: could not read -> {file_path}\n{err}");
            exit(1)
        });

        todos.append(&mut todo::parce_todo(file_path, &content, todo_regex_exp));
    } else if file_type.is_dir() {
        // for dir

        // dir to ignore
        let ignore_dires = vec![".git", "debug", "bin", "target", "build"];
        let files =
            extract_all_file_from_folder(PathBuf::from(path), ignore_dires).expect("ERROR:");

        for file in files {
            let content: String = read_file_content_as_string(&file).unwrap_or_else(|_| {
                // eprintln!("ERROR: could not read -> {file}\n{err}");
                "".to_string()
            });
            todos.append(&mut todo::parce_todo(&file, &content, todo_regex_exp));
        }
    } else {
        eprint!("Can't deceted {path}");
        exit(10);
    }

    //check is sort option is passed or not
    if args.len() > 2 && (args.contains(&"-s".to_string()) || args.contains(&"--sort".to_string()))
    {
        todo::sort_todos(&mut todos);
    }

    todo::print_todo(todos);
    Ok(())
}
