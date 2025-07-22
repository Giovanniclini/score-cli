pub const PLAYER_FIELD_COUNT: usize = 1;

#[derive(Debug)]
pub struct Player {
    player_name: String
}

impl Player {
    pub fn from_name(player_name: String) -> Player {
        Player { player_name: player_name }
    }

    pub fn get_name(&self) -> &str {
        &self.player_name
    }
}