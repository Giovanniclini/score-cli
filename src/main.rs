use std::{env, process};

use score_cli::commands::Command;

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();

    let command =  match Command::parse(&args) {
        Ok(command) => command,
        Err(err) => {
            eprintln!("Probblem parsing command: {err}");
            process::exit(1);
        }
    };

    if let Err(err) = command.run() {
        eprintln!("Problem executing command: {err}");
        process::exit(1);
    }

}