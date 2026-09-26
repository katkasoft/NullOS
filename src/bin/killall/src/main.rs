use std::{env, fs, io};
use rustix::process::{kill_process, Pid, Signal};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: killall [-signal] [process]");
        std::process::exit(1);
    }
    let mut process = String::new();
    let mut sig: i32 = 15;
    for arg in args.iter() {
        if let Some(rest) = arg.strip_prefix('-') {
            sig = rest.parse().unwrap();
        } else {
            process = arg.clone();
        }
    }
    let sig = match Signal::from_raw(sig) {
        Some(s) => s,
        None => {
            eprintln!("Invalid numeric signal number: {}", sig);
            std::process::exit(1);
        }
    };
    let mut found = false;
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Ok(pid) = name.parse::<i32>() {
                let comm = match fs::read_to_string(format!("/proc/{}/comm", pid)) {
                    Ok(n) => n,
                    Err(_) => continue,
                };
                if comm.trim() == process {
                    let pid = match Pid::from_raw(pid) {
                        Some(p) => p,
                        None => continue,
                    };
                    if let Err(e) = kill_process(pid, sig) {
                        eprintln!("Failed to kill process: {}", e);
                        continue;
                    }
                    found = true;
                }
            }
        }
    }
    if !found {
        eprintln!("{}: no process found", process);
        std::process::exit(1);
    }
    Ok(())
}