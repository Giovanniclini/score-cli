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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_path_with_some_dir() {
        let dir = String::from("/tmp/test_dir");
        let path_elements = vec!["subfolder", "file.json"];
        let result = create_path(&path_elements, Some(&dir));

        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(
            path,
            PathBuf::from("/tmp/test_dir").join("subfolder").join("file.json")
        );
    }

    #[test]
    fn test_create_path_with_none_dir() {
        let current_dir = env::current_dir().unwrap();
        let path_elements = vec!["subfolder", "file.json"];
        let result = create_path(&path_elements, None);

        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, current_dir.join("subfolder").join("file.json"));
    }

    #[test]
    fn test_create_path_with_empty_path() {
        let dir = String::from("/tmp/test_dir");
        let result = create_path(&[], Some(&dir));

        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, PathBuf::from("/tmp/test_dir"));
    }

    #[test]
    fn test_create_path_none_dir_empty_path() {
        let current_dir = env::current_dir().unwrap();
        let result = create_path(&[], None);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), current_dir);
    }
}
