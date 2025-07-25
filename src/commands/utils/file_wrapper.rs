use crate::commands::utils::storage::Storage;
use crate::commands::utils::utils::create_path;
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek};
use serde::{Serialize, de::DeserializeOwned};

pub struct FileWrapper {
    path: PathBuf,
    file: File
}

pub struct FileWrapperOptions {
    read: bool,
    write: bool,
    create: bool
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

        let data_serialized = serde_json::to_string(entity).unwrap();
        self.file.set_len(0).map_err(|e| format!("Truncation error: {}", e))?;
        self.file.rewind().map_err(|e| format!("Rewind error: {}", e))?;
        self.file.write_all(data_serialized.as_bytes())
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
        self.file.rewind().map_err(|e| format!("rewind error: {}", e))?;
    
        match self.file.read_to_string(&mut existing_data) {
            Ok(_) => {
                Ok(existing_data)
            },
            Err(_) => Err(format!("An error occurred reading file: {}", self.path.display()))
        }
    }
}

impl FileWrapper {
    pub fn from_string(file_name: &str, dir: Option<&String>, options: FileWrapperOptions) -> Result<FileWrapper, String> {

        let file_path = create_path(file_name, dir)?;

        let players_file = OpenOptions::new()
            .read(options.read)
            .write(options.write)
            .create(options.create)
            .open(&file_path);

        match players_file {
            Ok(file) => Ok(FileWrapper{path: file_path, file: file}),
            Err(_err) => Err(format!("An error occured while trying to open: {}", file_path.display()))
        }
    }

}

