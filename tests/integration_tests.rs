#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_version_flag() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rosenv"));
}

#[test]
fn test_help_flag() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "ROS 2 distribution environment manager",
        ));
}

#[test]
fn test_list_command_short() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("list")
        .arg("--short")
        .assert()
        .success();
}

#[test]
fn test_list_command_names_only() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("list")
        .arg("--names-only")
        .assert()
        .success();
}

#[test]
fn test_status_command() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("status")
        .assert()
        .success();
}

#[test]
fn test_activate_missing_distro() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("activate")
        .arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_deactivate_command() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("deactivate")
        .assert()
        .success()
        .stdout(predicate::str::contains("unset ROS_DISTRO"));
}

#[test]
fn test_init_zsh() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("init")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::str::contains("rosenv()"))
        .stdout(predicate::str::contains("rosenv init zsh"));
}

#[test]
fn test_init_bash() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("init")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::str::contains("rosenv()"))
        .stdout(predicate::str::contains("rosenv init bash"));
}

#[test]
fn test_info_missing_distro() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("info")
        .arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_doctor_command() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Checking ROS 2 environment"));
}

#[test]
fn test_invalid_command() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_init_generates_valid_shell_function() {
    let output = Command::cargo_bin("rosenv")
        .unwrap()
        .arg("init")
        .arg("zsh")
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("rosenv()"));
    assert!(stdout.contains("case \"$1\" in"));
    assert!(stdout.contains("activate)"));
    assert!(stdout.contains("deactivate)"));
    assert!(stdout.contains("status)"));
    assert!(stdout.contains("command rosenv"));
}

#[test]
fn test_deactivate_generates_cleanup_script() {
    let output = Command::cargo_bin("rosenv")
        .unwrap()
        .arg("deactivate")
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("unset ROS_DISTRO"));
    assert!(stdout.contains("unset AMENT_PREFIX_PATH"));
    assert!(stdout.contains("PATH="));
}

#[test]
fn test_list_with_both_flags() {
    Command::cargo_bin("rosenv")
        .unwrap()
        .arg("list")
        .arg("--short")
        .arg("--names-only")
        .assert()
        .success();
}
