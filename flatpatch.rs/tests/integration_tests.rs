use assert_cmd::Command;
use std::fs::{File};
use predicates::str::contains;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_root_check() {
    // Mock the environment variable USER to simulate running as a non-root user
    std::env::set_var("USER", "non-root");

    let mut cmd = Command::cargo_bin("flatpatch").unwrap();
    cmd.assert()
        .failure()
        .stderr(contains("Please run with sudo privileges."));
}

#[test]
fn test_os_check() {
    // Mock reading from /etc/os-release to simulate a non-Ubuntu system
    let dir = tempdir().unwrap();
    let os_release_path = dir.path().join("os-release");
    let mut os_release_file = File::create(&os_release_path).unwrap();
    writeln!(os_release_file, "ID=fedora").unwrap();

    // Mock the environment to read from our temp directory
    std::env::set_var("USER", "root");
    std::env::set_var("OS_RELEASE_PATH", os_release_path.to_str().unwrap());

    let mut cmd = Command::cargo_bin("flatpatch").unwrap();
    cmd.assert()
        .failure()
        .stderr(contains("Only Ubuntu is supported."));
}

#[test]
fn test_snap_remove() {
    // Simulate a system with installed Snap package ('hello')
    std::env::set_var("USER", "root");

    let mut cmd = Command::cargo_bin("flatpatch").unwrap();
    cmd.assert()
        .success()
        .stdout(contains("Successfully removed snap 'hello'"));
}

#[test]
fn test_flatpak_install() {
    std::env::set_var("USER", "root");

    let mut cmd = Command::cargo_bin("flatpatch").unwrap();
    cmd.assert()
        .success()
        .stdout(contains("Successfully installed Flatpak and plugin"));
}

