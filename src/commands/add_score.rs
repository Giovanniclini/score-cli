use crate::commands::{TIME_OPTIONAL_ARGUMENT, SAVE_DIR_OPTIONAL_ARGUMENT};
use crate::commands::models::player;
use crate::commands::models::game;
use crate::commands::utils::{file_wrapper::FileWrapper, file_wrapper::FileWrapperOptions, storage::Storage};
use std::collections::HashMap;

const ADMITTED_OPTIONAL_ARGUMENTS: [&str; 2] = [SAVE_DIR_OPTIONAL_ARGUMENT, TIME_OPTIONAL_ARGUMENT];

#[derive(Debug)]
pub struct AddScore {
    game: game::Game,
    optional_args: HashMap<String, String>
}

impl AddScore {
    pub fn create(args: &[String], optional_args: &HashMap<String, String>) -> Result<AddScore, String> {

        for (key, _) in optional_args {
            if !ADMITTED_OPTIONAL_ARGUMENTS.contains(&key.as_str()) {
                return Err(format!("Unknown optional command for add-score {}.", key));
            }
        }
        
        let new_game = game::Game::build(args[0].clone(), args[1..].to_vec(), optional_args.get(TIME_OPTIONAL_ARGUMENT).map(|s| s.as_str()))?;

        Ok(AddScore {
            game: new_game,
            optional_args: optional_args.to_owned()
        })
    }

    pub fn run(&self) -> Result<(), String> {

        let data_file_path = self.optional_args.get(SAVE_DIR_OPTIONAL_ARGUMENT);

        self.check_players_existance()?;

        let file_name = format!("{}.json", self.game.get_name());

        let file_options = FileWrapperOptions::default();
        let mut file = FileWrapper::from_string(&file_name, data_file_path, file_options)?;

        if file.is_empty()? {
            let games = game::Games::from_games(vec![self.game.clone()]);
            file.save(&games)?;
        } else {
            let mut games: game::Games = file.load()?;
            games.add_game(self.game.clone());
            file.save(&games)?;
        }
        println!("Added game of {} with id: {}.", self.game.get_name(), self.game.get_id());

        Ok(())
    }

    fn check_players_existance(&self) -> Result<(), String> {
        let data_file_path = self.optional_args.get(SAVE_DIR_OPTIONAL_ARGUMENT);

        let file_options = FileWrapperOptions::default();
        let mut player_file = FileWrapper::from_string(player::FILE_NAME_DATA, data_file_path, file_options)?;

        if player_file.is_empty()? {
            return Err("No Players' data found.".to_string());
        } else {
            let players: player::Players = player_file.load()?;
            for (player, _) in self.game.get_scores() {
                let player_to_check = player::Player::new(player.to_string());
                if !players.exists(player_to_check) {
                    return Err(format!("Player {} does not exist.", player));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_valid_input() {
        let args = vec![
            "Catan".to_string(),
            "player1-name::10".to_string(),
            "player2-name::8".to_string(),
        ];
        let mut optional_args = HashMap::new();
        optional_args.insert(TIME_OPTIONAL_ARGUMENT.to_string(), "2025-07-25".to_string());

        let result = AddScore::create(&args, &optional_args);
        assert!(result.is_ok());

        let add_score = result.unwrap();
        assert_eq!(add_score.game.get_name(), "Catan");
        assert_eq!(add_score.optional_args.get(TIME_OPTIONAL_ARGUMENT), Some(&"2025-07-25".to_string()));
    }

    #[test]
    fn test_create_with_unknown_optional_arg() {
        let args = vec!["Catan".to_string(), "player1-name::10".to_string()];
        let mut optional_args = HashMap::new();
        optional_args.insert("--unknown".to_string(), "value".to_string());

        let result = AddScore::create(&args, &optional_args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown optional command for add-score --unknown.");
    }

    #[test]
    fn test_create_game_build_failure() {
        let args = vec!["Catan".to_string()];
        let optional_args = HashMap::new();

        let result = AddScore::create(&args, &optional_args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No scores provided."));
    }
}
