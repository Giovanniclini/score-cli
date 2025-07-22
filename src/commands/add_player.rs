use crate::commands::models::player;

#[derive(Debug)]
pub struct AddPlayer {
    player: player::Player
}

impl AddPlayer {
    pub fn parse(args: &[String]) -> Result<AddPlayer, &'static str> {
        if args.len() != player::PLAYER_FIELD_COUNT {
            return Err("Invalid number of arguments for add-player.")
        }
        
        let new_player = player::Player::from_name(args[0].clone());

        Ok(AddPlayer {
            player: new_player
        })
    }

    pub fn run(&self) -> Result<(), &'static str> {
        println!("Added player: {}", self.player.get_name());
        Ok(())
    }
}