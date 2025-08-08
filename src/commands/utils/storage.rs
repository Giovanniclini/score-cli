use serde::{Serialize, de::DeserializeOwned};

pub trait Storage {
    fn is_empty(&mut self) -> Result<bool, String>;
    fn save(&mut self, entity: &impl Serialize) -> Result<(), String>;
    fn load<T: DeserializeOwned>(&mut self) -> Result<T, String>;
    fn get_data(&mut self) -> Result<String, String>;
}
