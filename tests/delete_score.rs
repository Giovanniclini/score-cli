use assert_cmd::Command;
use predicates::{prelude::*, str::contains};
use regex::Regex;
use tempfile::tempdir;

#[test]
fn test_add_and_delete_score() {
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
    let assert = cmd
        .args(&[
            "add-score",
            "catan",
            "player-name1::10",
            "player-name2::20",
            "--time",
            "2025-01-01",
            "--save-dir",
            temp_path,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added game of catan with id:"));

    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let re = Regex::new(r"(?i)id: ([0-9a-fA-F\-]{36})").unwrap();
    let caps = re.captures(&output).expect("UUID not found in output");
    let uuid = &caps[1];

    let mut delete_cmd = Command::cargo_bin("score-cli").unwrap();
    delete_cmd
        .args(&["delete-score", uuid, "--save-dir", temp_path])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed game with id"));
}

#[test]
fn test_delete_score_id_not_present() {
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
    let assert = cmd
        .args(&[
            "add-score",
            "catan",
            "player-name1::10",
            "player-name2::20",
            "--time",
            "2025-01-01",
            "--save-dir",
            temp_path,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added game of catan with id:"));

    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let re = Regex::new(r"(?i)id: ([0-9a-fA-F\-]{36})").unwrap();
    let caps = re.captures(&output).expect("UUID not found in output");
    let _uuid = &caps[1];

    let mut delete_cmd = Command::cargo_bin("score-cli").unwrap();
    delete_cmd
        .args(&[
            "delete-score",
            "3990e3b4-123d-4d7f-9461-335269897805",
            "--save-dir",
            temp_path,
        ])
        .assert()
        .failure()
        .stderr(contains(
            "Game with id 3990e3b4-123d-4d7f-9461-335269897805 not found.",
        ));
}

#[test]
fn test_delete_score_id_not_well_formatted() {
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
    let assert = cmd
        .args(&[
            "add-score",
            "catan",
            "player-name1::10",
            "player-name2::20",
            "--time",
            "2025-01-01",
            "--save-dir",
            temp_path,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added game of catan with id:"));

    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let re = Regex::new(r"(?i)id: ([0-9a-fA-F\-]{36})").unwrap();
    let caps = re.captures(&output).expect("UUID not found in output");
    let _uuid = &caps[1];

    let mut delete_cmd = Command::cargo_bin("score-cli").unwrap();
    delete_cmd
        .args(&["delete-score", "id-not-uuid", "--save-dir", temp_path])
        .assert()
        .failure()
        .stderr(contains("Impossible to decode id id-not-uuid."));
}
