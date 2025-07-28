use crate::commands::{SAVE_DIR_OPTIONAL_ARGUMENT};
use crate::commands::models::player;
use crate::commands::utils::{file_wrapper::FileWrapper, file_wrapper::FileWrapperOptions, storage::Storage};
use std::collections::HashMap;

const FILE_NAME_DATA: &str = "players.json";

const ADMITTED_OPTIONAL_ARGUMENTS: [&str; 1] = [SAVE_DIR_OPTIONAL_ARGUMENT];

#[derive(Debug)]
pub struct DeletePlayer {
    player: player::Player,
    optional_args: HashMap<String, String>
}

impl DeletePlayer {
    pub fn create(args: &[String], optional_args: &HashMap<String, String>) -> Result<DeletePlayer, String> {

        for (key, _) in optional_args {
            if !ADMITTED_OPTIONAL_ARGUMENTS.contains(&key.as_str()) {
                return Err(format!("Unknown optional command for delete-player {}.", key));
            }
        }

        if args.len() != player::PLAYER_FIELD_COUNT {
            return Err("Invalid number of arguments for delete-player.".to_string())
        }
        
        let new_player = player::Player::new(args[0].clone());

        Ok(DeletePlayer {
            player: new_player,
            optional_args: optional_args.to_owned()
        })
    }

    pub fn run(&self) -> Result<(), String> {
        let players_file_path = self.optional_args.get(SAVE_DIR_OPTIONAL_ARGUMENT);

        let file_options = FileWrapperOptions::default();
        let mut file = FileWrapper::from_string(&[FILE_NAME_DATA], players_file_path, file_options)?;

        if file.is_empty()? {
            return Err("No players' data found.".to_string());
        } else {
            let mut players: player::Players = file.load()?;
            players.remove_player(self.player.clone())?;
            file.save(&players)?;
        }
        println!("Deleted player: {}.", self.player.get_name());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_valid_input() {
        let args = vec!["player-name".to_string()];
        let mut optional_args = HashMap::new();
        optional_args.insert(SAVE_DIR_OPTIONAL_ARGUMENT.to_string(), "some/path".to_string());

        let result = DeletePlayer::create(&args, &optional_args);
        assert!(result.is_ok());

        let command = result.unwrap();
        assert_eq!(command.player.get_name(), "player-name");
        assert_eq!(command.optional_args.get(SAVE_DIR_OPTIONAL_ARGUMENT), Some(&"some/path".to_string()));
    }

    #[test]
    fn test_create_with_unknown_optional_arg() {
        let args = vec!["player-name".to_string()];
        let mut optional_args = HashMap::new();
        optional_args.insert("--unknown".to_string(), "value".to_string());

        let result = DeletePlayer::create(&args, &optional_args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Unknown optional command for delete-player --unknown."
        );
    }

    #[test]
    fn test_create_with_missing_arguments() {
        let args: Vec<String> = vec![]; 
        let optional_args = HashMap::new();

        let result = DeletePlayer::create(&args, &optional_args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid number of arguments for delete-player.");
    }

    #[test]
    fn test_create_with_extra_arguments() {
        let args = vec!["Alice".to_string(), "Extra".to_string()];
        let optional_args = HashMap::new();

        let result = DeletePlayer::create(&args, &optional_args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid number of arguments for delete-player.");
    }
}
