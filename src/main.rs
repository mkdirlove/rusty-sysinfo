extern crate sys_info;
use chrono::Utc;
use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::net::TcpListener;use std::process::exit;
use sys_info::{
    disk_info, hostname, linux_os_release, mem_info, os_release, DiskInfo, LinuxOSReleaseInfo,
    MemInfo,
};
mod custom;
use custom::*;

fn save_logs_to_file(file_path: &str, logs: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(logs.as_bytes())?;
    Ok(())
}

fn check_open_ports(start_port: u16, end_port: u16) -> HashMap<u16, String> {
    let mut open_ports = HashMap::new();

    let service_port_mappings: HashMap<u16, &str> = [
        (80, "HTTP"),
        (443, "HTTPS"),
        (22, "SSH"),
        // Add more port-to-service mappings here
    ]
    .iter()
    .cloned()
    .collect();

    for port in start_port..=end_port {
        if TcpListener::bind(("0.0.0.0", port)).is_ok() {
            if let Some(service_name) = service_port_mappings.get(&port) {
                open_ports.insert(port, String::from(*service_name));
            } else {
                open_ports.insert(port, String::from("Unknown service"));
            }
        }
    }

    open_ports
}

fn main() {
    //// HOST INFORMATION
    println!("-=-= HOST INFORMATION =-=-");
    println!("Hostname: {}", hostname().unwrap_or_default());
    println!("Date: {}", Utc::now());
    let release_info: LinuxOSReleaseInfo = match linux_os_release() {
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
        Ok(release_info) => release_info,
    };

    println!(
        "OS: {} ({})",
        release_info.pretty_name.unwrap_or_default(),
        os_release().unwrap_or_default()
    );

    //// OPEN PORTS
    let open_ports_info = "-=-= OPEN PORTS =-=-\n";
    println!("{}", open_ports_info);
    let open_ports = check_open_ports(1, 65535);
    if open_ports.is_empty() {
        println!("No open ports found.");
    } else {
        let mut log_content = String::new();
        log_content.push_str("Open Ports:\n");
        for (port, service_name) in &open_ports {
            let line = format!("{} -> {}\n", port, service_name);
            println!("{}", line);
            log_content.push_str(&line);
        }

        // Save logs to a file
        if let Err(err) = save_logs_to_file("logs.txt", &log_content) {
            eprintln!("Failed to save logs to file: {}", err);
        } else {
            println!("Logs saved to 'logs.txt'");
        }
    }
    
    //// LINUX MEMORY INFORMATION
    println!("\n\n-=-= MEMORY INFORMATION =-=-");
    let mem_info: MemInfo = match mem_info() {
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
        Ok(mem_info) => mem_info,
    };

    println!(
        "Total Memory: {} bytes\nFree Memory: {} bytes\nAvailable: {} bytes\nSwap: {} bytes",
        &mem_info.total, &mem_info.free, &mem_info.avail, &mem_info.swap_total
    );

    //// DISK INFORMATION
    println!("\n\n-=-= DISK INFORMATION =-=-");
    let disk_info: DiskInfo = match disk_info() {
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
        Ok(disk_info) => disk_info,
    };

    println!(
        "Total Disk Space: {} bytes\nFree Disk Space: {} bytes",
        &disk_info.total, &disk_info.free
    );

    //// BATTER INFORMATION
    println!("\n\n-=-= BATTERY INFORMATION =-=-");
    println!(
        "Battery Capacity: {}\nBattery Status: {}",
        battery_amount(),
        battery_status()
    );

    //// ENVIRONMENT INFORMATION
    println!("\n\n-=-= ENVIRONMENT VARIABLES =-=-");
    for (var, val) in env::vars() {
        println!("{}={}", var, val);
    }

    //// EXTRA INFORMATION
    println!("\n\n-=-= OTHER INFORMATION =-=-");
    println!("Is ASLR Enabled? : {}", if aslr() { "Yes" } else { "No" });
    println!("PATH: {}", env::var("PATH").unwrap_or_default());
    println!("Docker? : {}", if in_docker() { "yes" } else { "No" });


    //// TIMERS, CRON JOBS, SERVICES
    println!("\n\n-=-= TIMERS, CRONJOBS, SERVICES =-=-");
    println!("Cron Jobs: {}\n\n", if crontab().is_empty() {String::from("No cronjobs for current user.")} else {crontab()} );
    println!("Timers: \n{}\n\n", timers());
    println!("Services: \n{}\n\n", sysd_services());
}
