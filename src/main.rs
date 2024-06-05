use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    // let website_to_block = "9gag.com";
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <website_to_block>", args[0]);
        std::process::exit(1);
    }

    let website_to_block = &args[1];
    let hosts_path = "/etc/hosts";

    // Check if the website is already blocked
    if !is_website_blocked(hosts_path, website_to_block)? {
        // If the website is not blocked, add it to the hosts file
        block_website(hosts_path, website_to_block)?;
        println!("Website {} blocked successfully.", website_to_block);
    } else {
        println!("Website {} is already blocked.", website_to_block);
    }

    Ok(())
}

fn is_website_blocked(hosts_path: &str, website: &str) -> io::Result<bool> {
    let file = File::open(hosts_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(website) {
            return Ok(true);
        }
    }

    Ok(false)
}

fn block_website(hosts_path: &str, website: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(hosts_path)?;

    writeln!(file, "127.0.0.1\t{}", website)?;
    writeln!(file, "::1\t{}", website)?;

    Ok(())
}
