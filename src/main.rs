use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() -> io::Result<()> {
    // List of websites to block
    let websites = vec![
        "reddit.com",
        "9gag.com"
    ];

    // Path to the hosts file
    let hosts_path = "/private/etc/hosts";

    // Backup the original hosts file
    let backup_path = "/private/etc/hosts.backup";

    Command::new("sudo")
        .arg("cp")
        .arg(hosts_path)
        .arg(backup_path)
        .status()
        .expect("Failed to backup hosts file");

    // Open the hosts file with write access
    let mut hosts_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(Path::new(hosts_path))
        .expect("Failed to open hosts file");

    // Write the entries to the hosts file
    for website in &websites {
        writeln!(hosts_file, "127.0.0.1 {}", website)?;
        writeln!(hosts_file, "127.0.0.1 www.{}", website)?;
    }

    println!("Websites blocked successfully.");
    Ok(())
}
