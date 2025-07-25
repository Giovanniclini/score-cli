use crate::commands::models::player;
use crate::commands::utils::file_wrapper::{FileWrapper, FileWrapperOptions};
use std::collections::HashMap;

const FILE_NAME_DATA: &str = "players.json";

#[derive(Debug)]
pub struct AddPlayer {
    player: player::Player,
    optional_args: HashMap<String, String>
}

impl AddPlayer {
    pub fn parse(args: &[String], optional_args: &HashMap<String, String>) -> Result<AddPlayer, String> {
        if args.len() != player::PLAYER_FIELD_COUNT {
            return Err("Invalid number of arguments for add-player.".to_string())
        }
        
        let new_player = player::Player::new(args[0].clone());

        Ok(AddPlayer {
            player: new_player,
            optional_args: optional_args.to_owned()
        })
    }

    pub fn run(&self) -> Result<(), String> {
        let players_file_path = self.optional_args.get("--save-dir");

        let file_options = FileWrapperOptions::default();
        let mut file = FileWrapper::from_string(FILE_NAME_DATA, players_file_path, file_options)?;

        if file.is_empty()? {
            let players = player::Players::from_players(vec![self.player.clone()]);
            file.serialize_to_file(&players)?;
        } else {
            let mut players: player::Players = file.deserialize_from_file()?;
            players.add_player(self.player.clone());
            file.serialize_to_file(&players)?;
        }
        println!("Added player {}.", self.player.get_name());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    
}