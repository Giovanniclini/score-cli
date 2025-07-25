use crate::commands::models::player;
use crate::commands::utils::{file_wrapper::FileWrapper, file_wrapper::FileWrapperOptions, storage::Storage};
use std::collections::HashMap;

const FILE_NAME_DATA: &str = "players.json";

#[derive(Debug)]
pub struct DeletePlayer {
    player: player::Player,
    optional_args: HashMap<String, String>
}

impl DeletePlayer {
    pub fn parse(args: &[String], optional_args: &HashMap<String, String>) -> Result<DeletePlayer, String> {
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
        let players_file_path = self.optional_args.get("--save-dir");

        let file_options = FileWrapperOptions::default();
        let mut file = FileWrapper::from_string(FILE_NAME_DATA, players_file_path, file_options)?;

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