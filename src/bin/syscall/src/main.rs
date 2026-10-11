use std::{env, fs, io};
use libc::{syscall, c_long};

fn main() -> io::Result<()> {
    let raw_args: Vec<String> = env::args().collect();
    if raw_args.len() >= 2 && raw_args[1] == "--help" {
        let name = std::path::Path::new(&raw_args[0])
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| raw_args[0].clone());
        let filename = format!("/etc/help/{}.txt", name);
        let contents = fs::read_to_string(&filename)?;
        println!("{}", contents);
        std::process::exit(0);
    }
    let args: Vec<String> = raw_args.into_iter().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: syscall [number] [args...]");
        std::process::exit(1);
    }
    let num: c_long = args[0].parse().unwrap_or_else(|_| {
        eprintln!("syscall: invalid number: {}", args[0]);
        std::process::exit(1);
    });
    let call_args: Vec<c_long> = args[1..].iter()
        .map(|s| s.parse::<c_long>().unwrap_or(0))
        .collect();
    let result = unsafe {
        match call_args.len() {
            0 => syscall(num),
            1 => syscall(num, call_args[0]),
            2 => syscall(num, call_args[0], call_args[1]),
            3 => syscall(num, call_args[0], call_args[1], call_args[2]),
            4 => syscall(num, call_args[0], call_args[1], call_args[2], call_args[3]),
            5 => syscall(num, call_args[0], call_args[1], call_args[2], call_args[3], call_args[4]),
            6 => syscall(num, call_args[0], call_args[1], call_args[2], call_args[3], call_args[4], call_args[5]),
            _ => {
                eprintln!("syscall: too many arguments (max 6)");
                std::process::exit(1);
            }
        }
    };
    println!("{}", result);
    Ok(())
}