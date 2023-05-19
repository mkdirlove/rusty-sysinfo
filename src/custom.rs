use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

pub fn battery_amount() -> String {
    match read_to_string("/sys/class/power_supply/BAT0/capacity") {
        Ok(s) => s.trim().to_owned(),
        Err(_) => "N/A".to_owned(),
    }
}

pub fn battery_status() -> String {
    match read_to_string("/sys/class/power_supply/BAT0/status") {
        Ok(s) => s.trim().to_owned(),
        Err(_) => "N/A".to_owned(),
    }
}

pub fn aslr() -> bool {
    match read_to_string("/proc/sys/kernel/randomize_va_space") {
        Ok(s) => match s.as_str().trim() {
            "0" => return false,
            "1" => return true,
            "2" => return true,
            _ => {
                eprintln!("Couldn't figure out ASLR\nExpected: 1/0, Found: {}", s);
                return false;
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Couldn't figure out ASLR");
            false
        }
    }
}

// TODO: Implement this
#[allow(dead_code)]
pub fn virtual_machine() -> bool {
    false
}

pub fn in_docker() -> bool {
    let docker_env = Path::new("/.dockerenv");

    docker_env.exists()
}

pub fn crontab() -> String {
    String::from_utf8(Command::new("crontab").arg("-l").output().unwrap().stdout)
        .unwrap()
        .trim()
        .to_owned()
}

pub fn timers() -> String {
    String::from_utf8(
        Command::new("systemctl")
            .arg("list-timers")
            .arg("--all")
            .output()
            .unwrap()
            .stdout
    )
    .unwrap()
    .trim()
    .to_owned()
}

pub fn sysd_services() -> String {
    String::from_utf8(
        Command::new("systemctl")
            .arg("list-unit-files")
            .output()
            .unwrap()
            .stdout
    )
    .unwrap()
    .trim()
    .to_owned()
}