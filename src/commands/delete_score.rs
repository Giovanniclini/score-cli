use crate::commands::SAVE_DIR_OPTIONAL_ARGUMENT;
use crate::commands::models::game;
use crate::commands::utils::utils::create_path;
use crate::commands::utils::{
    file_wrapper::FileWrapper, file_wrapper::FileWrapperOptions, storage::Storage,
};
use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, io};
use uuid::Uuid;

const ADMITTED_OPTIONAL_ARGUMENTS: [&str; 1] = [SAVE_DIR_OPTIONAL_ARGUMENT];
const GAMES_FOLER: &str = "games";

#[derive(Debug)]
pub struct DeleteScore {
    game_id: String,
    optional_args: HashMap<String, String>,
}

impl DeleteScore {
    pub fn create(
        args: &[String],
        optional_args: &HashMap<String, String>,
    ) -> Result<DeleteScore, String> {
        for (key, _) in optional_args {
            if !ADMITTED_OPTIONAL_ARGUMENTS.contains(&key.as_str()) {
                return Err(format!(
                    "Unknown optional command for delete-score {}.",
                    key
                ));
            }
        }

        if args.len() != 1 {
            return Err("Invalid number of arguments for delete-score.".to_string());
        }

        Ok(DeleteScore {
            game_id: args[0].clone(),
            optional_args: optional_args.to_owned(),
        })
    }

    pub fn run(&self) -> Result<(), String> {
        let data_file_path = self.optional_args.get(SAVE_DIR_OPTIONAL_ARGUMENT);

        let file_options = FileWrapperOptions::default();
        let game_dir = create_path(&[GAMES_FOLER], data_file_path)?;

        let game_files = fs::read_dir(&game_dir)
            .map_err(|_| "An error occurred while accessing the data.")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .map_err(|_| "An error occurred whil accessing the data.")?;

        for game_file in game_files {
            let mut file = FileWrapper::from_path(game_file, file_options.clone())?;
            let mut games: game::Games = file.load()?;
            let uuid = Uuid::from_str(&self.game_id)
                .map_err(|_| format!("Impossible to decode id {}.", self.game_id))?;
            if let Ok(game) = games.delete(uuid) {
                file.save(&games)?;
                println!("Removed game with id {}.", game.get_id());
                return Ok(());
            }
        }

        Err(format!("Game with id {} not found.", self.game_id))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_create_valid_input() {
        let args = vec!["game-id".to_string()];
        let optional_args = HashMap::new();

        let result = DeleteScore::create(&args, &optional_args);
        assert!(result.is_ok());

        let add_score = result.unwrap();
        assert_eq!(add_score.game_id, "game-id");
    }

    #[test]
    fn test_create_no_arguments() {
        let args = vec![];
        let optional_args = HashMap::new();

        let result = DeleteScore::create(&args, &optional_args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid number of arguments for delete-score."
        );
    }

    #[test]
    fn test_create_no_admitted_arguments() {
        let args = vec![];
        let mut optional_args = HashMap::new();
        optional_args.insert("--unknown".to_string(), "value".to_string());

        let result = DeleteScore::create(&args, &optional_args);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Unknown optional command for delete-score --unknown."
        );
    }
}
