use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Duration;

pub fn local_device_discovery() {
    let host_ip_response = Command::new("ip")
    .arg("-4")
    .arg("addr")
    .arg("show")
    .arg("|")
    .arg("grep")
    .arg("-v")
    .arg("'inet 127.0.0.1'")
    .arg("|")
    .arg("grep")
    .arg("-Po")
    .arg(r"'inet \K[\d.]+'")
    .output()
    .expect("Failed to execute arp command").stdout;
    let host_ip_address = String::from_utf8_lossy(&host_ip_response);
    let div_host_ip_address: Vec<&str> = host_ip_address.split(".").collect();

    let mut network_address: String = "192.168.1".to_string();

    if div_host_ip_address.len() >= 3 {
        network_address = format!("{}.{}.{}", div_host_ip_address[0], div_host_ip_address[1], div_host_ip_address[2]);
    } else{
        //ADD CODE FOR MANUAL INPUT
    }

    println!("Clearing arp table");
    for i in 1..=254 {
        let ip = format!("{}.{}", network_address, i);
        let _ = Command::new("arp")
            .arg("-d")
            .arg(&ip)
            .stdout(Stdio::null())
            .status();
    }

    println!("Populating arp table");
    let mut handles = Vec::new();

    for i in 1..=254 {
        let ip = format!("{}.{}", network_address, i);
        let handle = Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg(&ip)
            .arg("-f")
            .arg("-i")
            .arg("0.002")
            .stdout(Stdio::null())
            .spawn()
            .expect("Failed to spawn ping command");
        handles.push(handle);
    }

    for mut handle in handles {
        handle.wait().expect("Ping command failed");
    }

    println!("Cleaning arp table");
    for i in 1..=254 {
        let ip = format!("{}.{}", network_address, i);
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
                .output();
        }
    }

    let arp_table = Command::new("arp")
    .arg("-e")
    .output()
    .expect("Failed to execute arp command").stdout;

    println!("{}", String::from_utf8_lossy(&arp_table));
}