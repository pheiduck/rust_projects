use std::process::Command;
use which::which;

#[test]
fn test_which_curl() {
    // Testet, ob das 'curl' Kommando auf dem System vorhanden ist
    let curl_path = which("curl");
    assert!(curl_path.is_ok(), "curl should be installed on the system");
}

#[test]
fn test_uname_command() {
    // Testet, ob der uname Befehl erfolgreich ausgeführt wird
    let output = Command::new("uname")
        .arg("-s")
        .output()
        .expect("Failed to execute uname command");

    assert!(output.status.success(), "uname command should succeed");
    let uname_str = String::from_utf8_lossy(&output.stdout);
    assert!(uname_str.contains("Linux"), "uname should return 'Linux'");
}
