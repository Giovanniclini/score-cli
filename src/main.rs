use std::{env, process};

use score_cli::commands::Command;
use score_cli::commands::add_player::AddPlayer;
fn main() {

    let args: Vec<String> = env::args().skip(1).collect();

    let command = match args.get(0).map(|s| s.as_str()) {
        Some("add-player") => match AddPlayer::parse(&args[1..]) {
            Ok(cmd) => Command::AddPlayer(cmd),
            Err(err) => Command::Invalid(err)
        }
        _ => Command::Invalid("Invalid or missing command.")
    };

    if let Err(err) = command.run() {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    }

}