use std::env;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, exit};

fn main() {
    // Variables
    let bindir = "/var/lib/airconnect/";
    let sysdir = "/etc/systemd/system/";

    // Check Privileges
    if env::var("USER").unwrap() != "root" {
        println!("\x1b[1;33mWARNING:\x1b[0m Please run with sudo privileges.");
        exit(1);
    }

    // Check OS
    let uname = Command::new("uname")
        .arg("-s")
        .output()
        .expect("Failed to execute uname command");

    let uname_str = String::from_utf8_lossy(&uname.stdout);

    if uname_str.trim() != "Linux" {
        println!("\x1b[0;31mERROR:\x1b[0m You are trying to install this on an unsupported System.\nThis Installer is only supported by Linux Distributions (w/ systemd).");
        exit(1);
    }

    // Check if the directory exists
    if fs::metadata(bindir).is_ok() {
        println!("\x1b[1;32mUpdate AirConnect...\x1b[0m");

        Command::new("systemctl")
            .arg("stop")
            .arg("airupnp")
            .status()
            .expect("Failed to stop airupnp service");

        env::set_current_dir(bindir).expect("Failed to change directory");

        let latest_release_url = Command::new("curl")
            .arg("--compressed")
            .arg("--silent")
            .arg("https://api.github.com/repos/philippe44/AirConnect/releases/latest")
            .output()
            .expect("Failed to fetch latest release");

        let download_url = String::from_utf8_lossy(&latest_release_url.stdout)
            .lines()
            .find(|line| line.contains("browser_download_url"))
            .unwrap()
            .split('"')
            .nth(3)
            .unwrap();

        Command::new("curl")
            .arg("--compressed")
            .arg("--progress-bar")
            .arg("-Lo")
            .arg("airconnect.zip")
            .arg(download_url)
            .status()
            .expect("Failed to download airconnect.zip");

        Command::new("unzip")
            .arg("-p")
            .arg("airconnect.zip")
            .arg(format!("airupnp-linux-{}", uname_str.trim()))
            .output()
            .and_then(|output| {
                let mut file = fs::File::create("airupnp-linux")?;
                file.write_all(&output.stdout)?;
                Ok(())
            })
            .expect("Failed to unzip airconnect.zip");

        fs::remove_file("airconnect.zip").expect("Failed to remove airconnect.zip");
        fs::set_permissions("airupnp-linux", fs::Permissions::from_mode(0o755)).expect("Failed to set permissions");

        env::set_current_dir("/").expect("Failed to change directory back");

        Command::new("systemctl")
            .arg("daemon-reload")
            .status()
            .expect("Failed to reload daemon");

        Command::new("systemctl")
            .arg("start")
            .arg("airupnp")
            .status()
            .expect("Failed to start airupnp service");

        Command::new("systemctl")
            .arg("status")
            .arg("airupnp")
            .status()
            .expect("Failed to check status of airupnp service");
    } else {
        println!("\x1b[0;32mInstall AirConnect...\x1b[0m");

        Command::new("curl")
            .arg("--compressed")
            .arg("--progress-bar")
            .arg("-o")
            .arg("airupnp-linux")
            .arg("--create-dirs")
            .arg(format!("https://raw.githubusercontent.com/philippe44/AirConnect/c3dfcdb/bin/airupnp-linux-{}", uname_str.trim()))
            .arg("--output-dir")
            .arg(bindir)
            .status()
            .expect("Failed to download airupnp-linux");

        fs::set_permissions(format!("{}/airupnp-linux", bindir), fs::Permissions::from_mode(0o755)).expect("Failed to set permissions");

        Command::new("curl")
            .arg("-LO")
            .arg("https://raw.githubusercontent.com/pheiduck/rpi_configs/main/airconnect/airupnp.service")
            .current_dir(sysdir)
            .status()
            .expect("Failed to download airupnp.service");

        Command::new("systemctl")
            .arg("daemon-reload")
            .status()
            .expect("Failed to reload daemon");

        Command::new("systemctl")
            .arg("enable")
            .arg("--now")
            .arg("airupnp")
            .status()
            .expect("Failed to enable and start airupnp service");

        Command::new("systemctl")
            .arg("status")
            .arg("airupnp")
            .status()
            .expect("Failed to check status of airupnp service");
    }
}
