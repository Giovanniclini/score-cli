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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args() {
        let add_player = AddPlayer::parse(&vec![String::from("user-name")]);
        
        assert!(add_player.is_ok());

        let add_player = add_player.unwrap();
        let player_name = add_player.player.get_name();

        assert_eq!("user-name", player_name);
    }

    #[test]
    fn parse_args_zero_arguments() {
        let add_player = AddPlayer::parse(&vec![]);

        assert!(add_player.is_err())
    }

    #[test]
    fn parse_args_too_many_arguments() {
        let add_player = AddPlayer::parse(&vec![String::from("user-name1"), String::from("user-name2")]);

        assert!(add_player.is_err())
    }
}