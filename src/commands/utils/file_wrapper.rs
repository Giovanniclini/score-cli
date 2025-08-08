use crate::commands::utils::storage::Storage;
use crate::commands::utils::utils::create_path;
use serde::{Serialize, de::DeserializeOwned};
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Read, Seek, Write};
use std::path::PathBuf;

pub struct FileWrapper {
    path: PathBuf,
    file: File,
}

#[derive(Clone)]
pub struct FileWrapperOptions {
    read: bool,
    write: bool,
    create: bool,
}

impl FileWrapperOptions {
    pub fn default() -> Self {
        FileWrapperOptions {
            read: true,
            write: true,
            create: true,
        }
    }
}

impl Storage for FileWrapper {
    fn is_empty(&mut self) -> Result<bool, String> {
        let file_data = self.get_data()?;
        Ok(file_data.is_empty())
    }

    fn save(&mut self, entity: &impl Serialize) -> Result<(), String> {
        let data_serialized = serde_json::to_string_pretty(entity).unwrap();
        self.file
            .set_len(0)
            .map_err(|e| format!("Truncation error: {}", e))?;
        self.file
            .rewind()
            .map_err(|e| format!("Rewind error: {}", e))?;
        self.file
            .write_all(data_serialized.as_bytes())
            .map_err(|_: std::io::Error| format!("Error writing file: {}", self.path.display()))?;
        Ok(())
    }

    fn load<T: DeserializeOwned>(&mut self) -> Result<T, String> {
        let file_data = self.get_data()?;

        if file_data.trim().is_empty() {
            return Err(format!("File {} is empty.", self.path.display()));
        }

        let data_deserialized: T = match serde_json::from_str(&file_data) {
            Ok(data) => data,
            Err(_) => return Err(format!("Error deserializing file: {}", self.path.display())),
        };

        Ok(data_deserialized)
    }

    fn get_data(&mut self) -> Result<String, String> {
        let mut existing_data = String::new();
        self.file
            .rewind()
            .map_err(|e| format!("rewind error: {}", e))?;

        match self.file.read_to_string(&mut existing_data) {
            Ok(_) => Ok(existing_data),
            Err(_) => Err(format!(
                "An error occurred reading file: {}",
                self.path.display()
            )),
        }
    }
}

impl FileWrapper {
    pub fn from_string(
        path: &[&str],
        base_dir: Option<&String>,
        options: FileWrapperOptions,
    ) -> Result<FileWrapper, String> {
        let file_path = create_path(path, base_dir)?;

        if let Some(parent_dir) = file_path.parent() {
            create_dir_all(parent_dir).map_err(|e| {
                format!(
                    "Failed to create directories for path {}: {}",
                    file_path.display(),
                    e
                )
            })?;
        }

        let players_file = OpenOptions::new()
            .read(options.read)
            .write(options.write)
            .create(options.create)
            .open(&file_path);

        match players_file {
            Ok(file) => Ok(FileWrapper {
                path: file_path,
                file: file,
            }),
            Err(_err) => Err(format!(
                "An error occured while trying to open: {}",
                file_path.display()
            )),
        }
    }

    pub fn from_path(path: PathBuf, options: FileWrapperOptions) -> Result<FileWrapper, String> {
        if let Some(parent_dir) = path.parent() {
            create_dir_all(parent_dir).map_err(|e| {
                format!(
                    "Failed to create directories for path {}: {}",
                    path.display(),
                    e
                )
            })?;
        }

        let players_file = OpenOptions::new()
            .read(options.read)
            .write(options.write)
            .create(options.create)
            .open(&path);

        match players_file {
            Ok(file) => Ok(FileWrapper {
                path: path,
                file: file,
            }),
            Err(_err) => Err(format!(
                "An error occured while trying to open: {}",
                path.display()
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::io::Seek;
    use tempfile::tempdir;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct DummyData {
        name: String,
        value: i32,
    }

    fn create_test_filewrapper(file_name: &str) -> FileWrapper {
        let dir = tempdir().unwrap();
        let dir_path = dir.path().to_str().unwrap().to_string();

        FileWrapper::from_string(&[file_name], Some(&dir_path), FileWrapperOptions::default())
            .unwrap()
    }

    #[test]
    fn test_save_and_load() {
        let mut file = create_test_filewrapper("test_file.json");

        let data = DummyData {
            name: "Test".to_string(),
            value: 42,
        };

        file.save(&data).unwrap();

        let loaded: DummyData = file.load().unwrap();
        assert_eq!(data, loaded);
    }

    #[test]
    fn test_is_empty_true_on_new_file() {
        let mut file = create_test_filewrapper("empty.json");
        assert!(file.is_empty().unwrap());
    }

    #[test]
    fn test_is_empty_false_after_save() {
        let mut file = create_test_filewrapper("not_empty.json");

        let data = DummyData {
            name: "Filled".to_string(),
            value: 10,
        };
        file.save(&data).unwrap();

        assert!(!file.is_empty().unwrap());
    }

    #[test]
    fn test_get_data_error_on_invalid_file() {
        let mut file = create_test_filewrapper("invalid.json");

        file.file.set_len(0).unwrap();
        file.file.rewind().unwrap();
        file.file.write_all(b"{invalid json").unwrap();

        let result: Result<DummyData, _> = file.load();
        assert!(result.is_err());
    }
}
