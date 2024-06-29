// Copyright (c) Philip H.

#[allow(unused_imports)]
use users::Users;
use std::process::Command;

fn main() {
    // Check Privileges
    if user.name != "root"() {
        eprintln!("WARNING: Please run with sudo privileges.");
    } if else {
        let output = if cfg!(os_release != "Ubuntu") {
            eprintln!("ERROR: You are trying to install this on an unsupported distribution.\nOnly Ubuntu is supported.");
        } else {
            Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
        };
    }
}
