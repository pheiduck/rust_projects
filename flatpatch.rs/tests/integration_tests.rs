use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_root_check() {
    // Simulate running the command as a non-root user
    std::env::set_var("USER", "non-root");

    // Create the command to run the binary
    let mut cmd = Command::cargo_bin("flatpatch").expect("Binary not found");

    // Execute the command and check for the expected failure and stderr message
    cmd.assert()
        .failure()
        .stderr(contains("Please run with sudo privileges."));
}
