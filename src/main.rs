use std::{env, process};

use score_cli::commands::Command;

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();

    let command = Command::parse(&args);

    if let Err(err) = command.run() {
        println!("Problem executing command: {err}");
        process::exit(1);
    }

}