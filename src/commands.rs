use crate::commands::add_player::AddPlayer;

mod models;
pub mod add_player;

#[derive(Debug)]
pub enum Command<'a> {
    AddPlayer(AddPlayer),
    Invalid(&'a str)
}

impl<'a> Command<'a> {
    pub fn run(&self) -> Result<(), &'a str> {
        match &self {
            Command::AddPlayer(cmd) => cmd.run(),
            Command::Invalid(err) => Err(err)
        }
    }
}
