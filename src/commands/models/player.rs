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
    pub fn from_name(player_name: String) -> Player {
        Player { player_name: player_name }
    }

    pub fn get_name(&self) -> &str {
        &self.player_name
    }
}

impl Players {
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player.clone());
    }
}