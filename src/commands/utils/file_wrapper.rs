use std::env;
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

    pub fn is_empty(&mut self) -> Result<bool, String> {
        let file_data = self.get_data()?;
        Ok(file_data.is_empty())
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

    pub fn serialize_to_file(&mut self, entity: &impl Serialize) -> Result<(), String> {
        // TOFIX: always empty and writes the file. Append?
        // TODO: handle errors.
        let data_serialized = serde_json::to_string(entity).unwrap();
        self.file.set_len(0).unwrap();
        self.file.rewind().unwrap();
        match self.file.write_all(data_serialized.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Error writing file: {}", self.path.display()))
        }
    }

    pub fn deserialize_from_file<T: DeserializeOwned>(&mut self) -> Result<T, String> {
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
}

fn create_path(file_name: &str, dir: Option<&String>) -> Result<PathBuf, String> {
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