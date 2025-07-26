use std::env;
use std::path::PathBuf;

pub fn create_path(path: &[&str], dir: Option<&String>) -> Result<PathBuf, String> {
    let mut path_object = if let Some(dir) = dir {
        PathBuf::from(dir)
    } else {
        match env::current_dir() {
            Ok(p) => p,
            Err(_) => return Err("An error occurred while accessing the working folder.".to_string()),
        }
    };

    for el in path {
        path_object.push(el);
    }
    
    Ok(path_object)
}