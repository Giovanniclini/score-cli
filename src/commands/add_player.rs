use crate::commands::SAVE_DIR_OPTIONAL_ARGUMENT;
use crate::commands::models::player;
use crate::commands::models::player::FILE_NAME_DATA;
use crate::commands::utils::{
    file_wrapper::FileWrapper, file_wrapper::FileWrapperOptions, storage::Storage,
};
use std::collections::HashMap;

const ADMITTED_OPTIONAL_ARGUMENTS: [&str; 1] = [SAVE_DIR_OPTIONAL_ARGUMENT];

#[derive(Debug)]
pub struct AddPlayer {
    player: player::Player,
    optional_args: HashMap<String, String>,
}

impl AddPlayer {
    pub fn create(
        args: &[String],
        optional_args: &HashMap<String, String>,
    ) -> Result<AddPlayer, String> {
        for (key, _) in optional_args {
            if !ADMITTED_OPTIONAL_ARGUMENTS.contains(&key.as_str()) {
                return Err(format!("Unknown optional command for add-player {}.", key));
            }
        }

        if args.len() != player::PLAYER_FIELD_COUNT {
            return Err("Invalid number of arguments for add-player.".to_string());
        }

        let new_player = player::Player::new(args[0].clone());

        Ok(AddPlayer {
            player: new_player,
            optional_args: optional_args.to_owned(),
        })
    }

    pub fn run(&self) -> Result<(), String> {
        let players_file_path = self.optional_args.get(SAVE_DIR_OPTIONAL_ARGUMENT);

        let file_options = FileWrapperOptions::default();
        let mut file =
            FileWrapper::from_string(&[FILE_NAME_DATA], players_file_path, file_options)?;

        if file.is_empty()? {
            let players = player::Players::from_players(HashMap::from([(
                self.player.get_name().to_string(),
                self.player.clone(),
            )]));
            file.save(&players)?;
        } else {
            let mut players: player::Players = file.load()?;
            players.add_player(self.player.clone())?;
            file.save(&players)?;
        }
        println!("Added player {}.", self.player.get_name());

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
        optional_args.insert(
            SAVE_DIR_OPTIONAL_ARGUMENT.to_string(),
            "path/to/dir".to_string(),
        );

        let result = AddPlayer::create(&args, &optional_args);
        assert!(result.is_ok());

        let add_player = result.unwrap();
        assert_eq!(add_player.player.get_name(), "player-name");
        assert_eq!(
            add_player.optional_args.get(SAVE_DIR_OPTIONAL_ARGUMENT),
            Some(&"path/to/dir".to_string())
        );
    }

    #[test]
    fn test_create_invalid_number_of_args() {
        let args = vec![];
        let optional_args = HashMap::new();

        let result = AddPlayer::create(&args, &optional_args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid number of arguments for add-player."
        );
    }

    #[test]
    fn test_create_unknown_optional_argument() {
        let args = vec!["player-name".to_string()];
        let mut optional_args = HashMap::new();
        optional_args.insert("--unknown-flag".to_string(), "value".to_string());

        let result = AddPlayer::create(&args, &optional_args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Unknown optional command for add-player --unknown-flag."
        );
    }
}
