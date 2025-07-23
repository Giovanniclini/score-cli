use crate::commands::add_player::AddPlayer;

mod models;
mod utils;
mod add_player;

#[derive(Debug)]
pub enum Command<'a> {
    AddPlayer(Vec<String>),
    Invalid(&'a str)
}

impl<'a> Command<'a> {
    pub fn run(&self) -> Result<(), &'a str> {
        match &self {
            Command::AddPlayer(args) => {
                let command = AddPlayer::parse(&args)?;
                command.run()
            },
            Command::Invalid(err) => {
                Err(err)
            }
        }
    }

    pub fn parse(args: &[String]) -> Command {
        match args.get(0).map(|s| s.as_str()) {
            Some("add-player") => Command::AddPlayer(args[1..].to_vec()),
            _ => Command::Invalid("Unknown or missing command.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add_player() {
        let args = vec![String::from("add-player"), String::from("player-name")];
        let command = Command::parse(&args);
        if let Command::AddPlayer(inner_args) = command {
            assert_eq!(inner_args, vec!["player-name"]);
        } else {
            panic!("Expected Command::AddPlayer variant");
        }
    }

    #[test]
    fn parse_invalid() {
        let args = vec![String::from("invalid-command"), String::from("player-name")];
        let command = Command::parse(&args);
        if let Command::Invalid(err) = command {
            assert_eq!(err, "Unknown or missing command.");
        } else {
            panic!("Expected Command::Invalid variant");
        }
    }
}