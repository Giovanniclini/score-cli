use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cli_delete_player_removes_player_from_file() {
    let temp = tempdir().unwrap();
    let temp_path = temp.path().to_str().unwrap();

    let mut add_cmd = Command::cargo_bin("score-cli").unwrap();
    add_cmd
        .args(&["add-player", "player-name", "--save-dir", temp_path])
        .assert()
        .success();

    let mut del_cmd = Command::cargo_bin("score-cli").unwrap();
    del_cmd
        .args(&["delete-player", "player-name", "--save-dir", temp_path])
        .assert()
        .success();

    let player_file_path = temp.path().join("players.json");
    let content = fs::read_to_string(player_file_path).unwrap();
    assert!(!content.contains("player_name"));
}

#[test]
fn test_cli_delete_player_fails_player_not_present() {
    let temp = tempdir().unwrap();
    let temp_path = temp.path().to_str().unwrap();

    let mut add_cmd = Command::cargo_bin("score-cli").unwrap();
    add_cmd
        .args(&["add-player", "player-name", "--save-dir", temp_path])
        .assert()
        .success();

    let mut del_cmd = Command::cargo_bin("score-cli").unwrap();
    del_cmd
        .args(&[
            "delete-player",
            "different-player-name",
            "--save-dir",
            temp_path,
        ])
        .assert()
        .failure()
        .stderr(contains("Player different-player-name not found."));
}

#[test]
fn test_cli_delete_player_fails_no_data() {
    let temp = tempdir().unwrap();
    let temp_path = temp.path().to_str().unwrap();

    let mut del_cmd = Command::cargo_bin("score-cli").unwrap();
    del_cmd
        .args(&["delete-player", "player-name", "--save-dir", temp_path])
        .assert()
        .failure()
        .stderr(contains("No players' data found."));
}
