use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub const PLAYER_FIELD_COUNT: usize = 1;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    player_name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    players: HashMap<String, Player>
}

impl Player {
    pub fn new(player_name: String) -> Player {
        Player { player_name: player_name }
    }

    pub fn get_name(&self) -> &str {
        &self.player_name
    }
}

impl Players {

    pub fn from_players(players: HashMap<String, Player>) -> Players {
        Players {
            players: players
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.get_name().to_string(), player.clone());
    }

    pub fn remove_player(&mut self, player: Player) -> Result<(), String> {

        if let None = self.players.remove(player.get_name()) {
            return Err(format!("Player {} not found.", player.get_name()));
        }

        Ok(())

    }
}