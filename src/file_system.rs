use crate::Config;
use crate::{Result};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// list all the file inside a directory recursively
pub fn list_all_files(path: &Path, cfg: &Config) -> Result<Vec<PathBuf>> {
    let dir = match std::fs::read_dir(path) {
        Ok(read_dir) => read_dir,
        Err(err) => {
            if err.kind() == ErrorKind::NotADirectory {
                return Ok(vec![path.to_path_buf()]);
            }
            return Err(err.into());
        }
    };

    let mut files: Vec<PathBuf> = Vec::new();
    for entry in dir {
        let entry = entry?;

        //TODO: check if full path is necessary or not.
        let entry_path = entry.path(); //fs::canonicalize(entry.path())?;
        if entry_path.is_dir() {
            if cfg.dir_is_excluded(&entry_path) {
                continue;
            }
            let mut sub_files = match list_all_files(&entry_path, cfg) {
                Ok(files) => files,
                Err(err) => {
                    eprintln!("WARNING: Can't read dir `{d}`: {err}", d = entry_path.display());
                    continue;
                }
            };
            files.append(&mut sub_files);
        } else {
            files.push(entry_path);
        }
    }

    Ok(files)
}
