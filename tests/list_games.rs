use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

#[test]
fn test_list_games_shows_added_game() {
    let temp = tempdir().unwrap();
    let temp_path = temp.path().to_str().unwrap();

    Command::cargo_bin("score-cli")
        .unwrap()
        .args(&["add-player", "player1", "--save-dir", temp_path])
        .assert()
        .success();

    Command::cargo_bin("score-cli")
        .unwrap()
        .args(&["add-player", "player2", "--save-dir", temp_path])
        .assert()
        .success();

    Command::cargo_bin("score-cli")
        .unwrap()
        .args(&[
            "add-score",
            "catan",
            "player1::10",
            "player2::20",
            "--time",
            "2025-08-07",
            "--save-dir",
            temp_path,
        ])
        .assert()
        .success()
        .stdout(contains("Added game of catan"));

    Command::cargo_bin("score-cli")
        .unwrap()
        .args(&["list-games", "--save-dir", temp_path])
        .assert()
        .success()
        .stdout(contains("catan"))
        .stdout(contains("2025-08-07"))
        .stdout(contains("player1"))
        .stdout(contains("10"))
        .stdout(contains("player2"))
        .stdout(contains("20"));
}
