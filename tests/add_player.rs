use assert_cmd::Command;
use predicates::{prelude::PredicateBooleanExt, str::contains};
use tempfile::tempdir;
use std::fs;

#[test]
fn test_cli_add_player_creates_file_with_player() {
    let temp = tempdir().unwrap();
    let temp_path = temp.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("score-cli").unwrap();
    cmd.args(&["add-player", "player-name", "--save-dir", temp_path])
        .assert()
        .success();

    let player_file_path = temp.path().join("players.json");
    let content = fs::read_to_string(player_file_path).unwrap();
    assert!(content.contains("player-name"));
}

#[test]
fn test_cli_add_player_fails_on_missing_name() {
    let mut cmd = Command::cargo_bin("score-cli").unwrap();
    cmd.args(&["add-player"])
        .assert()
        .failure()
        .stderr(contains("Probblem parsing command:").or(contains("Problem executing command")));
}