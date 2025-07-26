use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_cli_add_score_creates_file_with_scores() {
    let temp = tempdir().unwrap();
    let temp_path = temp.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("score-cli").unwrap();
    cmd.args(&["add-player", "player-name1", "--save-dir", temp_path])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("score-cli").unwrap();
    cmd.args(&["add-player", "player-name2", "--save-dir", temp_path])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("score-cli").unwrap();
    cmd.args(&["add-score", "catan", "player-name1::10", "player-name2::20", "--save-dir", temp_path])
        .assert()
        .success();

    let score_file_path = temp.path().join("games").join("catan.json");
    let content = fs::read_to_string(score_file_path).unwrap();
    assert!(content.contains("player-name1"));
    assert!(content.contains("player-name2"));
}

#[test]
fn test_cli_add_score_creates_file_with_scores_and_time() {
    let temp = tempdir().unwrap();
    let temp_path = temp.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("score-cli").unwrap();
    cmd.args(&["add-player", "player-name1", "--save-dir", temp_path])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("score-cli").unwrap();
    cmd.args(&["add-player", "player-name2", "--save-dir", temp_path])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("score-cli").unwrap();
    cmd.args(&["add-score", "catan", "player-name1::10", "player-name2::20", "--time", "2025-01-01", "--save-dir", temp_path])
        .assert()
        .success();

    let score_file_path = temp.path().join("games").join("catan.json");
    let content = fs::read_to_string(score_file_path).unwrap();
    assert!(content.contains("player-name1"));
    assert!(content.contains("player-name2"));
    assert!(content.contains("2025-01-01"));
}

#[test]
fn test_cli_add_score_no_players_data() {
    let temp = tempdir().unwrap();
    let temp_path = temp.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("score-cli").unwrap();
    cmd.args(&["add-score", "catan", "player-name::10", "--save-dir", temp_path])
        .assert()
        .failure()
        .stderr(contains("No Players' data found."));
}

#[test]
fn test_cli_add_score_missing_player() {
    let temp = tempdir().unwrap();
    let temp_path = temp.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("score-cli").unwrap();
    cmd.args(&["add-player", "player-name2", "--save-dir", temp_path])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("score-cli").unwrap();
    cmd.args(&["add-score", "catan", "player-name1::10", "player-name2::20", "--time", "2025-01-01", "--save-dir", temp_path])
        .assert()
        .failure()
        .stderr(contains("Player player-name1 does not exist."));
}