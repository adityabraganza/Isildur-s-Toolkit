use std::process::Command;
use std::process::Stdio;
use std::thread;
use core::time::Duration;

use crate::functions::qol::sout;

pub fn local_device_discovery() {
    let current_terminal_id = Command::new("id").arg("-u").output().expect("Error in identifying root").stdout;
    if String::from_utf8_lossy(&current_terminal_id).trim() != "0"{
        sout("Please restart the program with root privilages");
        return;
    }

    let host_ip_response = Command::new("id")
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

    sout("Populating arp table");
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

    thread::sleep(Duration::from_secs(2)); // Waits for the ARP table to update

    for _i in 1..20{
        let _ = Command::new("arp")
        .arg("-e")
        .output()
        .expect("Failed to execute arp command").stdout;
    }

    let arp_table = Command::new("arp")
    .arg("-e")
    .output()
    .expect("Failed to execute arp command").stdout;

    sout(&String::from_utf8_lossy(&arp_table));
    sout("Done");
    sout("")
}