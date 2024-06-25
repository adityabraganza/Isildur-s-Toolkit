use std::io::{self, Write};
use std::net::Ipv4Addr;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    
    
    network = "192.168.1";

    // Check for removal all IPs in the arp table
    let mut clean_arp_table = false;
    loop {
        print!("Clean arp table (y/n): ");
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read input");
        match response.trim().to_lowercase().as_str() {
            "y" => {
                clean_arp_table = true;
                break;
            }
            "n" => break,
            _ => println!("Invalid response. Please enter 'y' or 'n'."),
        }
    }

    println!("Clearing arp table");
    for i in 1..=254 {
        let ip = format!("{}.{}", network, i);
        let _ = Command::new("arp")
            .arg("-d")
            .arg(&ip)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }

    println!("Populating arp table");
    for i in 1..=254 {
        let ip = format!("{}.{}", network, i);
        let _ = Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg(&ip)
            .arg("-f")
            .arg("-i")
            .arg("0.002")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    }

    for _ in 1..=254 {
        thread::sleep(Duration::from_millis(4));
    }

    println!("Cleaning arp table");
    for i in 1..=254 {
        let ip = format!("{}.{}", network, i);
        let output = Command::new("arp")
            .arg("-a")
            .arg(&ip)
            .output()
            .expect("Failed to execute arp command");
        if String::from_utf8_lossy(&output.stdout).starts_with("arp") {
            let _ = Command::new("arp")
                .arg("-d")
                .arg(&ip)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }

    let _ = Command::new("arp")
        .arg("-e")
        .output()
        .expect("Failed to execute arp command");

    println!("Done");
}