// Copyright (c) Philip H.

use std::env;
use std::process::exit;

fn main() {
    // Check Privileges
    if env::var("USER").unwrap() != "root" {
        println!("\x1b[1;33mWARNING: \x1b[0mPlease run with sudo privileges.");
        exit(1);
    }
}
