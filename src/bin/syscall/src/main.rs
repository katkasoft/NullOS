use std::{env, fs, io};
use std::ffi::CString;
use libc::{syscall, c_long};

fn parse_arg(arg: &str) -> (c_long, Option<CString>) {
    if arg.starts_with('"') && arg.ends_with('"') && arg.len() >= 2 {
        let inner = &arg[1..arg.len() - 1];
        let processed = inner.replace("\\n", "\n");
        let cstr = CString::new(processed).unwrap();
        (cstr.as_ptr() as c_long, Some(cstr))
    } else {
        (arg.parse::<c_long>().unwrap_or(0), None)
    }
}

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
    let accepted_params: Vec<String> = vec!["-q".to_string()];
    let mut params: Vec<String> = vec![];
    let mut syscall_args: Vec<String> = vec![];
    for arg in args.iter() {
        if arg.starts_with("-") {
            for chr in arg.chars().skip(1) {
                if accepted_params.contains(&chr.to_string()) {
                    params.push(chr.to_string());
                } else {
                    eprintln!("No such parametre: {}", chr);
                }
            }
        } else {
            syscall_args.push(arg.clone());
        }
    }
    let num: c_long = syscall_args[0].parse().unwrap_or_else(|_| {
        eprintln!("syscall: invalid number: {}", syscall_args[0]);
        std::process::exit(1);
    });
    let mut values: Vec<c_long> = Vec::new();
    let mut strings: Vec<CString> = Vec::new();
    for arg in &syscall_args[1..] {
        let (val, s) = parse_arg(arg);
        values.push(val);
        if let Some(s) = s {
            strings.push(s);
        }
    }
    let result = unsafe {
        match values.len() {
            0 => syscall(num),
            1 => syscall(num, values[0]),
            2 => syscall(num, values[0], values[1]),
            3 => syscall(num, values[0], values[1], values[2]),
            4 => syscall(num, values[0], values[1], values[2], values[3]),
            5 => syscall(num, values[0], values[1], values[2], values[3], values[4]),
            6 => syscall(num, values[0], values[1], values[2], values[3], values[4], values[5]),
            _ => {
                eprintln!("syscall: too many arguments");
                std::process::exit(1);
            }
        }
    };
    drop(strings);
    if result < 0 && result >= -4095 {
        eprintln!("errno: {}", -result);
        std::process::exit(1);
    } else if !params.contains(&"q".to_string()) {
        println!("{}", result);
    }
    Ok(())
}