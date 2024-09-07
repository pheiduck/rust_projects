use assert_cmd::Command;
use mockito::mock;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_root_check() {
    // Mock the environment variable USER to simulate running as a non-root user
    std::env::set_var("USER", "non-root");

    let mut cmd = Command::cargo_bin("your_program_name").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Please run with sudo privileges."));
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

    let mut cmd = Command::cargo_bin("your_program_name").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Only Ubuntu is supported."));
}

#[test]
fn test_snap_remove() {
    // Mock the snap list command to simulate installed snaps
    let snap_list_mock = mock("GET", "/snap/list")
        .with_status(200)
        .with_body("Name  Version   Rev    Tracking       Publisher   Notes\n\
                    hello  2.10.1    29     latest/stable  canonical✓  -")
        .create();

    // Ensure the root privileges and OS are mocked as valid
    std::env::set_var("USER", "root");
    std::env::set_var("OS_RELEASE_PATH", "/etc/os-release");

    let mut cmd = Command::cargo_bin("your_program_name").unwrap();
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Successfully removed snap 'hello'"));

    snap_list_mock.assert();
}

#[test]
fn test_flatpak_install() {
    // Mock the flatpak installation process
    let mock_flatpak_repo = mock("POST", "/add-apt-repository")
        .with_status(200)
        .create();

    let mock_flatpak_install = mock("POST", "/apt-get")
        .with_status(200)
        .create();

    std::env::set_var("USER", "root");
    let mut cmd = Command::cargo_bin("your_program_name").unwrap();

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Successfully installed Flatpak and plugin"));

    mock_flatpak_repo.assert();
    mock_flatpak_install.assert();
}
