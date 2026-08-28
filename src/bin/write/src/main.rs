use std::{io, env};
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let accepted_params: Vec<String> = vec![];
    let mut params: Vec<String> = vec![];
    let mut filtered: Vec<String> = vec![];
    if args.len() == 1 {
        eprintln!("Usage: write [file] [content]");
        std::process::exit(1);
    }
    for arg in args.iter().skip(1) {
        if arg.starts_with("-") {
            for chr in arg.chars().skip(1) {
                if accepted_params.contains(&chr.to_string()) {
                    params.push(chr.to_string());
                } else {
                    eprintln!("No such parametre: {}", chr);
                }
            }
        } else {
            filtered.push(arg.clone());
        }
    }
    if filtered.len() == 1 {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filtered[0].clone())?;
        println!("Write v0.1. Enter q to exit");
        loop {
            let mut input = String::new();
            print!(">");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if input == "q\n" {
                break
            }
            if let Err(e) = writeln!(file, "{}", input) {
                eprintln!("Error while writing to file: {}", e);
            }
        }
    } else {
        let content = filtered[1..].join(" ");
        if let Err(e) = std::fs::write(&filtered[0], content) {
            eprintln!("Error while writing to file: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}
