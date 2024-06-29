// Copyright (c) Philip H.

use std::env;
use std::process::exit;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    // Check Privileges
    if env::var("USER").unwrap() != "root" {
        println!("\x1b[1;33mWARNING: \x1b[0mPlease run with sudo privileges.");
        exit(1);
    }
    // Check OS
    let os_release_contents = std::fs::read_to_string("/etc/os-release")
        .expect("Failed to read /etc/os-release file");

    if !os_release_contents.contains("ID=ubuntu") {
        let red = "\u{001b}[0;31m";
        let nc = "\u{001b}[0m";
        eprintln!("{}ERROR:{} You are trying to install this on an unsupported distribution.\nOnly Ubuntu is supported.", red, nc);
        std::process::exit(1);
    } else {

    let red = "\u{001b}[0;31m";
    let nc = "\u{001b}[0m";

    println!("{}Uninstall snapd...{}", red, nc);

    // Create the nosnap.pref file in /etc/apt/preferences.d/
    let mut file = File::create("/etc/apt/preferences.d/nosnap.pref")
        .expect("Failed to create nosnap.pref file");

    let content = r#"
        # To prevent repository packages from triggering the installation of snap,
        # this file forbids snapd from being installed by APT.
        
        Package: snapd
        Pin: release a=*
        Pin-Priority: -10
        "#;

    file.write_all(content.as_bytes())
        .expect("Failed to write content to nosnap.pref file");

    // Remove all installed snaps except core
    let output = Command::new("snap")
        .arg("remove")
        .arg("--purge")
        .arg(
            Command::new("snap")
                .arg("list")
                .output()
                .expect("Failed to get list of installed snaps")

        )
        .output()
        .expect("Failed to remove installed snaps");

    if !output.status.success() {
        eprintln!("Error removing installed snaps: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Purge snapd and gnome-software-plugin-snap
    let output = Command::new("apt-get")
        .arg("purge")
        .arg("-y")
        .arg("snapd")
        .arg("gnome-software-plugin-snap")
        .output()
        .expect("Failed to purge snapd and gnome-software-plugin-snap");

    if !output.status.success() {
        eprintln!("Error purging snapd and gnome-software-plugin-snap: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Remove snap-related directories
    let _ = Command::new("rm")
        .arg("-rfv")
        .arg("~/snap")
        .arg("/snap")
        .arg("/var/snap")
        .arg("/var/lib/snapd")
        .arg("/var/cache/snapd")
        .output()
        .expect("Failed to remove snap-related directories");

    // Reload systemd daemon
    let _ = Command::new("systemctl")
        .arg("daemon-reload")
        .output()
        .expect("Failed to reload systemd daemon");

    }

}
