use tabled::Table;
use crate::commands::{SAVE_DIR_OPTIONAL_ARGUMENT, add_score::GAMES_FOLER};
use crate::commands::models::game::{self, Games};
use crate::commands::utils::{file_wrapper::FileWrapper, file_wrapper::FileWrapperOptions, storage::Storage, utils::create_path};
use std::{fs, io, collections::HashMap};

const ADMITTED_OPTIONAL_ARGUMENTS: [&str; 1] = [SAVE_DIR_OPTIONAL_ARGUMENT];

#[derive(Debug)]
pub struct ListGames {
    optional_args: HashMap<String, String>
}

impl ListGames {
    pub fn create(_args: &[String], optional_args: &HashMap<String, String>) -> Result<ListGames, String> {

        for (key, _) in optional_args {
            if !ADMITTED_OPTIONAL_ARGUMENTS.contains(&key.as_str()) {
                return Err(format!("Unknown optional command for list-games {}.", key));
            }
        }
        
        Ok(ListGames {
            optional_args: optional_args.to_owned()
        })
    }

    pub fn run(&self) -> Result<(), String> {
        let data_file_path = self.optional_args.get(SAVE_DIR_OPTIONAL_ARGUMENT);

        let file_options = FileWrapperOptions::default();
        let game_dir = create_path(&[GAMES_FOLER], data_file_path)?;

        let game_files =  fs::read_dir(&game_dir).map_err(|_| "An error occurred while accessing the data.")?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>().map_err(|_| "An error occurred whil accessing the data.")?;

        let mut all_games = Games::create_empy();
        for game_file in game_files {
            let mut file = FileWrapper::from_path(game_file, file_options.clone())?;
            let games_curr: game::Games = file.load()?;
            all_games.extend(&games_curr);
        }

        let all_games_vec = all_games.order_by_date();
        let table_rows = game::from_vec_to_game_rows(all_games_vec);
        let table = Table::new(table_rows).to_string();
        println!("{}", table);

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    //use super::*;

    
}
