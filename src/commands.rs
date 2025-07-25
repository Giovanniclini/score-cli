use crate::commands::{add_player::AddPlayer, delete_player::DeletePlayer};
use std::collections::HashMap;

mod models;
mod utils;
mod add_player;
mod delete_player;

#[derive(Debug)]
enum CommandType {
    AddPlayer,
    DeletePlayer,
    Invalid
}

const OPTIONAL_ARGUMENTS: [&str; 1] = ["--save-dir"];

pub struct Command {
    command: CommandType,
    args: Vec<String>,
    optional_args: HashMap<String, String>
}

impl Command {

    fn new(command_type: CommandType, args: Vec<String>, optional_args: HashMap<String, String>) -> Command {
        Command {
            command: command_type,
            args: args,
            optional_args: optional_args
        }
    }

    fn get_args(&self) -> Vec<String> {
        self.args.to_vec()
    }

    fn get_optional_args(&self) -> HashMap<String, String> {
        self.optional_args.to_owned()
    }

    pub fn run(&self) -> Result<(), String> {
        match &self.command {
            CommandType::AddPlayer => {
                let command = AddPlayer::parse(&self.get_args(), &self.get_optional_args())?;
                command.run()
            },
            CommandType::DeletePlayer => {
                let command = DeletePlayer::parse(&self.get_args(), &self.get_optional_args())?;
                command.run()
            },
            CommandType::Invalid => {
                Err("Invalid or missing command.".to_string())
            }
        }
    }

    pub fn parse(args: &[String]) -> Result<Command, String> {
        let command_type = match args.get(0).map(|s| s.as_str()) {
            Some("add-player") => CommandType::AddPlayer,
            Some("delete-player") => CommandType::DeletePlayer,
            _ => CommandType::Invalid
        };

        let (args, opt_args) = parse_args(&args[1..])?;

        Ok(Command::new(command_type, args, opt_args))

    }
}

fn parse_args(args: &[String]) -> Result<(Vec<String>, HashMap<String, String>), String> {

    let positional_args = get_positional_args(args);
    let optional_start = positional_args.len();
    let optional_args = get_optional_args(&args[optional_start..])?;

    Ok((positional_args, optional_args))
}

fn get_positional_args(args: &[String]) -> Vec<String> {
    let mut positional_args = Vec::new();
    let mut i = 0;
    while i < args.len() {

        if args[i].as_str().starts_with("--") {
            break;
        } else {
            positional_args.push(args[i].clone());
            i+=1;
        }
    }

    positional_args
}

fn get_optional_args(args: &[String]) -> Result<HashMap<String, String>, String> {
    let mut optional_args = HashMap::new();

    let mut i = 0;
    while i < args.len() {

        if OPTIONAL_ARGUMENTS.contains(&args[i].as_str()) {
            if i + 1 < args.len() {
                if args[i+1].as_str().starts_with("--") {
                    return Err(format!("Missing value for optional argument {}", args[i]))
                }
                optional_args.insert(args[i].clone(), args[i + 1].clone());
                i += 2;
            } else {
                return Err(format!("Missing value for optional argument {}", args[i]))
            }
        } else {
            return Err(format!("Unknown optional argument {}", args[i]))
        }
    }

    Ok(optional_args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_correct_input() {
        let args = vec![
            "player-name".to_string(),
            "--save-dir".to_string(),
            "path/to/dir".to_string(),
        ];

        let result = parse_args(&args);
        assert!(result.is_ok());

        let (positional, optional) = result.unwrap();
        assert_eq!(positional, vec!["player-name"]);
        assert_eq!(optional.get("--save-dir"), Some(&"path/to/dir".to_string()));
    }

    #[test]
    fn parse_args_empty_input() {
        let args = vec![];
        let result = parse_args(&args);
        assert!(result.is_ok());

        let (positional, optional) = result.unwrap();
        assert!(positional.is_empty());
        assert!(optional.is_empty());
    }

    #[test]
    fn parse_args_only_positional() {
        let args = vec!["player-name".to_string()];
        let result = parse_args(&args);
        assert!(result.is_ok());

        let (positional, optional) = result.unwrap();
        assert_eq!(positional, vec!["player-name"]);
        assert!(optional.is_empty());
    }

    #[test]
    fn parse_args_missing_optional_value() {
        let args = vec![
            "player-name".to_string(),
            "--save-dir".to_string(),
            "--other".to_string(),
        ];
        let result = parse_args(&args);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err, "Missing value for optional argument --save-dir");
    }

    #[test]
    fn parse_args_unknown_optional_argument() {
        let args = vec![
            "player-name".to_string(),
            "--unknown-flag".to_string(),
            "value".to_string(),
        ];
        let result = parse_args(&args);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err, "Unknown optional argument --unknown-flag");
    }

    #[test]
    fn get_positional_args_with_flag_in_middle() {
        let args = vec![
            "player-name1".to_string(),
            "--save-dir".to_string(),
            "player-name2".to_string(),
        ];
        let positional_args = get_positional_args(&args);
        assert_eq!(positional_args, vec!["player-name1"]);
    }

    #[test]
    fn get_positional_args_only_optional_arguments() {
        let args = vec!["--save-dir".to_string(), "path".to_string()];
        let positional_args = get_positional_args(&args);
        assert!(positional_args.is_empty());
    }

    #[test]
    fn get_positional_args_empty_input() {
        let args = vec![];
        let positional_args = get_positional_args(&args);
        assert_eq!(positional_args.len(), 0);
    }

    #[test]
    fn get_positional_args_standard_behaviour() {
        let args = vec![
            "player-name".to_string(),
            "--save-dir".to_string(),
            "path/to/dir".to_string(),
        ];

        let expected_positional_args = vec!["player-name".to_string()];
        let positional_args = get_positional_args(&args);

        assert_eq!(expected_positional_args.len(), positional_args.len());

        for (expected, actual) in expected_positional_args.iter().zip(positional_args.iter()) {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn get_positional_args_multiple_before_flag() {
        let args = vec![
            "player-name1".to_string(),
            "player-name2".to_string(),
            "--save-dir".to_string(),
            "path".to_string(),
        ];
        let positional_args = get_positional_args(&args);
        assert_eq!(positional_args, vec!["player-name1", "player-name2"]);
    }

    #[test]
    fn get_positional_args_no_optional_args() {
        let args = vec![
            "player-name1".to_string(),
            "player-name2".to_string()
        ];
        let positional_args = get_positional_args(&args);
        assert_eq!(positional_args, vec!["player-name1", "player-name2"]);
    }

    #[test]
    fn get_optional_args_unknown_optional_argument() {
        let args = vec!["--unknown-argument".to_string()];
        let optional_args_result = get_optional_args(&args);
    
        assert!(optional_args_result.is_err());

        let err_msg = optional_args_result.unwrap_err();

        assert_eq!(err_msg, "Unknown optional argument --unknown-argument");
    
    }  

    #[test]
    fn get_optional_args_missing_value_for_optional_arguments() {
        let args = vec!["--save-dir".to_string()];
        let optional_args_result = get_optional_args(&args);
    
        assert!(optional_args_result.is_err());

        let err_msg = optional_args_result.unwrap_err();

        assert_eq!(err_msg, "Missing value for optional argument --save-dir");
    
    }  

    #[test]
    fn get_optional_args_consecutive_optional_commands() {
        let args = vec!["--save-dir".to_string(), "--another-optional-command".to_string()];
        let optional_args_result = get_optional_args(&args);
    
        assert!(optional_args_result.is_err());

        let err_msg = optional_args_result.unwrap_err();

        assert_eq!(err_msg, "Missing value for optional argument --save-dir");
    
    }  

    #[test]
    fn get_optional_args_standard_behaviour() {
        let args = vec!["--save-dir".to_string(), "path/to/dir".to_string()];
        let optional_args_result = get_optional_args(&args);
    
        assert!(optional_args_result.is_ok());
    
        let optional_args = optional_args_result.unwrap();
        let save_dir = optional_args.get("--save-dir");
    
        assert!(save_dir.is_some());
        assert_eq!(save_dir.unwrap(), "path/to/dir");
    }    
}