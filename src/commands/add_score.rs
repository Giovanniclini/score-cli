use crate::commands::models::game;
use crate::commands::utils::{file_wrapper::FileWrapper, file_wrapper::FileWrapperOptions, storage::Storage};
use std::collections::HashMap;

#[derive(Debug)]
pub struct AddScore {
    game: game::Game,
    optional_args: HashMap<String, String>
}

impl AddScore {
    pub fn parse(args: &[String], optional_args: &HashMap<String, String>) -> Result<AddScore, String> {
        
        let new_game = game::Game::build(args[0].clone(), args[1..].to_vec(), optional_args.get("--time").map(|s| s.as_str()))?;

        Ok(AddScore {
            game: new_game,
            optional_args: optional_args.to_owned()
        })
    }

    pub fn run(&self) -> Result<(), String> {
        // TODO: Check for player existance
        let game_file_path = self.optional_args.get("--save-dir");
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