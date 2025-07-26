use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    game_name: String,
    scores: HashMap<String, usize>,
    time: NaiveDate
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Games {
    games: Vec<Game>
}

impl Game {
    pub fn build(gamename: String, scores: Vec<String>, time: Option<&str>) -> Result<Game, String> {

        let parsed_scores = Self::parse_scores(&scores)?;

        if parsed_scores.len() == 0 {
            return Err("No scores provided.".to_string());
        }

        let time = match time {
            Some(time) => NaiveDate::parse_from_str(&time, "%Y-%m-%d")
                                            .map_err(|_| "Error parsing date. The input format is YYYY-MM-DD.".to_string())?,
            None => Utc::now().date_naive()
        };

        Ok(Game {game_name: gamename, scores: parsed_scores, time: time})

    }

    fn parse_scores(scores: &[String]) -> Result<HashMap<String, usize>, String> {
        let mut hashed_scores = HashMap::new();
        dbg!(scores);
        for score in scores {
            let vec_score: Vec<&str> = score.as_str().split("::").collect();
            if vec_score.len() != 2 {
                return Err("Error parsing scores. The input format is player::score.".to_string());
            }
            let player = vec_score[0].to_string();
            let score: usize = vec_score[1].parse().map_err(|_| "Error parsing scores. The input format is player::score.".to_string())?;
            hashed_scores.insert(player, score);
        }



        Ok(hashed_scores)
    }

    pub fn get_name(&self) -> &str {
        &self.game_name
    }
}

impl Games {
    pub fn from_games(games: Vec<Game>) -> Games {
        Games{ games: games }
    }

    pub fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_game_error_parsing_time_string() {
        let game = Game::build("game-name".to_string(), vec!["player1::10".to_string(), "player2::20".to_string()], Some("impossible-to-parse"));
        assert!(game.is_err());
        assert_eq!(game.unwrap_err(), "Error parsing date. The input format is YYYY-MM-DD.");
    }

    #[test]
    fn build_game_well_formatted_data() {
        let game = Game::build("game-name".to_string(), vec!["player1::10".to_string(), "player2::20".to_string()], Some("2025-01-01"));
        assert!(game.is_ok());
        let game = game.unwrap();
        assert_eq!(game.game_name, "game-name");
        assert_eq!(game.scores.get("player1").unwrap(), &10);
        assert_eq!(game.scores.get("player1").unwrap(), &10);
        assert_eq!(game.time, NaiveDate::parse_from_str("2025-01-01", "%Y-%m-%d").unwrap());
    }

    #[test]
    fn test_parse_scores_valid_input() {
        let input = vec!["alice::10".to_string(), "bob::20".to_string()];
        let result = Game::parse_scores(&input);
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.get("alice"), Some(&10));
        assert_eq!(map.get("bob"), Some(&20));
    }

    #[test]
    fn test_parse_scores_invalid_format_single_colon() {
        let input = vec!["alice:10".to_string()];
        let result = Game::parse_scores(&input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Error parsing scores. The input format is player::score.");
    }

    #[test]
    fn test_parse_scores_invalid_number() {
        let input = vec!["alice::ten".to_string()];
        let result = Game::parse_scores(&input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Error parsing scores. The input format is player::score.");
    }

    #[test]
    fn test_parse_scores_empty_input() {
        let input: Vec<String> = vec![];
        let result = Game::parse_scores(&input);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_parse_scores_too_many_colons() {
        let input = vec!["alice::10::bonus".to_string()];
        let result = Game::parse_scores(&input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Error parsing scores. The input format is player::score.");
    }
}
