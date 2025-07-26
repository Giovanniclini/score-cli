use crate::commands::{TIME_OPTIONAL_ARGUMENT, SAVE_DIR_OPTIONAL_ARGUMENT};
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
        // TODO: Check for player existance
        let game_file_path = self.optional_args.get(SAVE_DIR_OPTIONAL_ARGUMENT);
        let file_name = format!("{}.json", self.game.get_name());

        let file_options = FileWrapperOptions::default();
        let mut file = FileWrapper::from_string(&file_name, game_file_path, file_options)?;

        if file.is_empty()? {
            let games = game::Games::from_games(vec![self.game.clone()]);
            file.save(&games)?;
        } else {
            let mut games: game::Games = file.load()?;
            games.add_game(self.game.clone());
            file.save(&games)?;
        }
        println!("Added game of {}.", self.game.get_name());

        Ok(())
    }
}