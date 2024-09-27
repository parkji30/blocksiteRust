use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    // let website_to_block = "9gag.com";
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <block|unblock> <website>", args[0]);
        std::process::exit(1);
    }

    let action = &args[1];
    let website = &args[2];
    let hosts_path = "/etc/hosts";

    match action.as_str() {
        "block" => {
            if !is_website_blocked(hosts_path, website)? {
                block_website(hosts_path, website)?;
                println!("Website {} blocked successfully.", website);
            } else {
                println!("Website {} is already blocked.", website);
            }
        }
        "unblock" => {
            if is_website_blocked(hosts_path, website)? {
                unblock_website(hosts_path, website)?;
                println!("Website {} unblocked successfully.", website);
            } else {
                println!("Website {} is not blocked.", website);
            }
        }
        _ => {
            eprintln!("Invalid action: {}. Use 'block' or 'unblock'.", action);
            std::process::exit(1);
        }
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


fn unblock_website(hosts_path: &str, website: &str) -> io::Result<()> {
    let file = File::open(hosts_path)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut is_blocked = false;

    for line in reader.lines() {
        let line = line?;
        if line.contains(website) {
            is_blocked = true; // Found the website to unblock, skip writing it back
            continue;
        }
        lines.push(line);
    }

    if is_blocked {
        // Rewrite the hosts file without the blocked website
        let mut file = OpenOptions::new().write(true).truncate(true).open(hosts_path)?;
        for line in lines {
            writeln!(file, "{}", line)?;
        }
    }

    Ok(())
}