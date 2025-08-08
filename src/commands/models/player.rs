use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const PLAYER_FIELD_COUNT: usize = 1;

pub const FILE_NAME_DATA: &str = "players.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    players: HashMap<String, Player>,
}

impl Player {
    pub fn new(player_name: String) -> Player {
        Player {
            player_name: player_name,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.player_name
    }
}

impl Players {
    pub fn from_players(players: HashMap<String, Player>) -> Players {
        Players { players: players }
    }

    pub fn add_player(&mut self, player: Player) -> Result<(), String> {
        if let Some(player) = self.players.get(player.get_name()) {
            return Err(format!("Player {} already exists.", player.get_name()));
        }

        self.players
            .insert(player.get_name().to_string(), player.clone());
        Ok(())
    }

    pub fn remove_player(&mut self, player: Player) -> Result<(), String> {
        if let None = self.players.remove(player.get_name()) {
            return Err(format!("Player {} not found.", player.get_name()));
        }

        Ok(())
    }

    pub fn exists(&self, player: Player) -> bool {
        if let Some(_) = self.players.get(player.get_name()) {
            true
        } else {
            false
        }
    }
}
