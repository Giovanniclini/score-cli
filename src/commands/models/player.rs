use serde::{Serialize, Deserialize};

pub const PLAYER_FIELD_COUNT: usize = 1;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    player_name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    players: Vec<Player>
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

    pub fn from_players(players: Vec<Player>) -> Players {
        Players {
            players: players
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player.clone());
    }

    pub fn remove_player(&mut self, player: Player) -> Result<(), String> {

        let mut i = 0;
        while i < self.players.len() {
            if self.players[i].get_name() == player.get_name() {
                self.players.remove(i);
                return Ok(());
            }
            i+=1;
        }
        
        Err(format!("Player {} not found.", player.get_name()))
    }
}