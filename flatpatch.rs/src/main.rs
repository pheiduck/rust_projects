// Copyright (c) Philip H.

use nix::unistd::Uid;
use std::process::Command;

    // Check Privileges
if !Uid::effective().is_root() {
    eprintln!("WARNING: \x1b[0mPlease run with sudo privileges.");
} else {
let output = if cfg!(os_relase != "Ubuntu") {
    eprintln!("ERROR: You are trying to install this on an unsupported distribution.\nOnly Ubuntu is supported.");
} else {
    Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process")
}
    };
