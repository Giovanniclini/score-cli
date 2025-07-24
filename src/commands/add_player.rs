use crate::commands::models::player;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::{Read, Write, Seek};

const FILE_NAME_DATA: &str = "players.json";

#[derive(Debug)]
pub struct AddPlayer {
    player: player::Player,
    optional_args: HashMap<String, String>
}

impl AddPlayer {
    pub fn parse(args: &[String], optional_args: &HashMap<String, String>) -> Result<AddPlayer, &'static str> {
        if args.len() != player::PLAYER_FIELD_COUNT {
            return Err("Invalid number of arguments for add-player.")
        }
        
        let new_player = player::Player::from_name(args[0].clone());

        Ok(AddPlayer {
            player: new_player,
            optional_args: optional_args.to_owned()
        })
    }

    pub fn run(&self) -> Result<(), &'static str> {
        //TODO: Check for player existance.
        let mut players_file_path = match self.optional_args.get("--save-dir") {
            Some(dir) => PathBuf::from(dir),
            None => env::current_dir().unwrap()
        };
        players_file_path.push(FILE_NAME_DATA);

        let mut players_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&players_file_path).unwrap();

        // Leggo il contenuto attuale (se presente)
        let mut existing_data = String::new();

        // TOFIX: If the file is empty, return Err.
        players_file.read_to_string(&mut existing_data).unwrap();

        println!("Contenuto attuale del file: {}", existing_data);

        let mut actual_players: player::Players = serde_json::from_str(&existing_data).unwrap();
        println!("deserialized = {:?}", actual_players);

        let new_player = player::Player::from_name(self.player.get_name().to_string());
        
        actual_players.add_player(new_player);

        let playyers_serialized = serde_json::to_string(&actual_players).unwrap();
        println!("serialized = {}", playyers_serialized);

        //TOFIX:: Can i append the new player?
        players_file.set_len(0).unwrap();
        players_file.rewind().unwrap();

        players_file.write_all(playyers_serialized.as_bytes()).unwrap();

        println!("Added player: {} in {}", self.player.get_name(), players_file_path.display());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    
}