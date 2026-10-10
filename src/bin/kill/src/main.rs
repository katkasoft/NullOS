use std::{env, io};
use rustix::process::{kill_process, Pid, Signal};
use std::fs;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == "--help" {
        let name = std::path::Path::new(&args[0])
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| args[0].clone());
        let filename = format!("/etc/help/{}.txt", name);
        let contents = fs::read_to_string(&filename)?;
        println!("{}", contents);
        std::process::exit(0)
    }
    let args: Vec<String> = args.into_iter().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: kill -[signal] [pid]");
        std::process::exit(1)
    }
    let mut pid: i32 = 0;
    let mut sig: i32 = 15;
    for arg in args.iter() {
        if let Some(rest) = arg.strip_prefix('-') {
            sig = rest.parse().unwrap();
        } else {
            pid = arg.parse().unwrap();
        }
    }
    let pid = match Pid::from_raw(pid) {
        Some(p) => p,
        None => {
            eprintln!("Invalid PID.");
            std::process::exit(1)
        }
    };
    let sig = match Signal::from_raw(sig) {
        Some(s) => s,
        None => {
            eprintln!("Invalid numeric signal number: {}", sig);
            std::process::exit(1)
        }
    };
    if let Err(e) = kill_process(pid, sig) {
        eprintln!("Failed to kill process: {}", e)
    }
    Ok(())
}