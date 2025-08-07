use serde::{Serialize, Deserialize};
use tabled::Tabled;
use std::collections::HashMap;
use chrono::prelude::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    id: Uuid,
    game_name: String,
    scores: HashMap<String, usize>,
    time: NaiveDate
}

#[derive(Tabled)]
pub struct GameRow {
    id: String,
    name: String,
    date: String,
    scores: String,
}

impl From<&Game> for GameRow {
    fn from(game: &Game) -> Self {

        let game_scores = game.get_scores()
            .iter()
            .map(|(player, score)| format!("{} {}", player, score))
            .collect::<Vec<_>>()
            .join(",");

        GameRow {
            id: game.get_id().to_string(),
            name: game.get_name().to_string(),
            date: game.get_datetime().to_string(),
            scores: game_scores
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Games {
    games: HashMap<Uuid, Game>
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

        Ok(Game {id: Uuid::new_v4(), game_name: gamename, scores: parsed_scores, time: time})

    }

    fn parse_scores(scores: &[String]) -> Result<HashMap<String, usize>, String> {
        let mut hashed_scores = HashMap::new();
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

    pub fn get_datetime(&self) -> &NaiveDate {
        &self.time
    }

    pub fn get_name(&self) -> &str {
        &self.game_name
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_scores(&self) -> &HashMap<String, usize> {
        &self.scores
    }
}

impl Games {
    pub fn from_games(games: HashMap<Uuid, Game>) -> Self {
        Self{ games: games }
    }

    pub fn create_empy() -> Self {
        Self {games: HashMap::new()}
    }

    pub fn get_games(&self) -> &HashMap<Uuid, Game> {
        &self.games
    }

    pub fn extend(&mut self, games: &Games) {
        self.games.extend(games.get_games().iter().map(|(id, game)| (id.clone(), game.clone())))
    }

    pub fn add_game(&mut self, game: Game) {
        self.games.insert(*game.get_id(), game);
    }

    pub fn delete(&mut self, game_id: Uuid) -> Result<Game, String> {
        if let Some(game) = self.games.remove(&game_id) {
            Ok(game)
        } else {
            Err("Game not found.".to_string())
        }
    }

    pub fn order_by_date(&self) -> Vec<Game> {
        let mut games = self.games.iter().map(|(_id, game)| game.clone()).collect::<Vec<_>>();
        games.sort_by_key(|g| g.time);
        games
    }

}

pub fn from_vec_to_game_rows(games: Vec<Game>) -> Vec<GameRow> {
    games.iter().map(|game| GameRow::from(game)).collect()
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
