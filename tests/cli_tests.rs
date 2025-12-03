//! Integration tests for CLI functionality

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("ytdl").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("YouTube Video Downloader"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("ytdl").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("ytdl"));
}

#[test]
fn test_examples_flag() {
    let mut cmd = Command::cargo_bin("ytdl").unwrap();
    cmd.arg("--examples")
        .assert()
        .success()
        .stdout(predicate::str::contains("Common Usage Examples"));
}

#[test]
fn test_invalid_url() {
    let mut cmd = Command::cargo_bin("ytdl").unwrap();
    cmd.arg("https://invalid-url.com")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_config_command() {
    let mut cmd = Command::cargo_bin("ytdl").unwrap();
    cmd.arg("config")
        .assert()
        .success()
        .stdout(predicate::str::contains("Current Configuration"));
}

#[test]
fn test_history_command() {
    let mut cmd = Command::cargo_bin("ytdl").unwrap();
    cmd.arg("history")
        .assert()
        .success(); // May show empty or existing history
}

#[test]
fn test_missing_url() {
    let mut cmd = Command::cargo_bin("ytdl").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No URL provided"));
}
