use std::{format, println};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    println!("{:<6} {:<8} {:<6} {:<5} {}", "PID", "USER", "TTY", "STAT", "CMD");
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Ok(pid) = name.parse::<u32>() {
                let file = File::open(format!("/proc/{}/status", pid)).expect("Failed to get uid");
                let reader = BufReader::new(file);
                let mut stat = String::new();
                let mut uid = String::new();
                for line in reader.lines() {
                    let line = line?;
                    if line.starts_with("State:") {
                        stat = line.split_whitespace().nth(1)
                            .unwrap_or("?")
                            .to_string();
                    } else if line.starts_with("Uid:") {
                        uid = line.split_whitespace().nth(1)
                            .unwrap_or("?")
                            .to_string();
                    }
                }
                let content = fs::read_to_string(format!("/proc/{}/stat", pid)).expect("Failed to get tty");
                let comm_end = content.rfind(')').unwrap_or(0);
                let rest = &content[comm_end + 1..];
                let tty_nr: i64 = rest.split_whitespace().nth(4)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let major = (tty_nr >> 8) & 0xfff;
                let minor = (tty_nr & 0xff) | ((tty_nr >> 12) & 0xfff00);
                let tty = if tty_nr == 0 {
                    "?".to_string()
                } else if major == 136 {
                    format!("pts/{}", minor)
                } else if major == 4 {
                    format!("tty{}", minor)
                } else if major == 5 {
                    format!("ttyS{}", minor)
                } else {
                    "?".to_string()
                };
                let cmdline = fs::read(format!("/proc/{}/cmdline", pid))
                    .unwrap_or_default();
                let cmd = if cmdline.is_empty() {
                    let comm = content
                        .split_once('(')
                        .and_then(|(_, rest)| rest.split_once(')'))
                        .map(|(name, _)| name.to_string())
                        .unwrap_or_else(|| "[unknown]".to_string());
                    format!("[{}]", comm)
                } else {
                    cmdline
                        .split(|&b| b == 0)
                        .filter(|s| !s.is_empty())
                        .map(|s| String::from_utf8_lossy(s).into_owned())
                        .collect::<Vec<_>>()
                        .join(" ")
                };
                println!("{:<6} {:<8} {:<6} {:<5} {}", pid, uid, tty, stat, cmd);
            }
        }
    }
    Ok(())
}
