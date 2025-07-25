use std::env;
use std::path::PathBuf;

pub fn create_path(file_name: &str, dir: Option<&String>) -> Result<PathBuf, String> {
    let mut path = if let Some(dir) = dir {
        PathBuf::from(dir)
    } else {
        match env::current_dir() {
            Ok(p) => p,
            Err(_) => return Err("An error occurred while accessing the working folder.".to_string()),
        }
    };

    path.push(file_name);
    Ok(path)
}