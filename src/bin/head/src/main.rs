use std::env;
use std::fs::File;
use std::io::{self, BufRead};
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
    let accepted_params: Vec<String> = vec!["n".to_string()];
    let mut params: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    if args.len() == 1 {
        eprintln!("Usage: ");
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
            files.push(arg.clone());
        }
    }
    if params.is_empty() {
        let count = 10;
        for file in files.iter() {
            let file_opened = File::open(file)?;
            let reader = io::BufReader::new(file_opened);
            let mut i = 0;
            for line in reader.lines() {
                if i >= count {
                    break;
                }
                let line = line?;
                println!("{}", line);
                i += 1;
            }
        }
    } else if params.contains(&"n".to_string()) {
        if files.len() != 2 {
            eprintln!("Usage: head [files] -n [lines]");
            std::process::exit(1);
        }
        let count: u16 = files[1].parse().unwrap();
        let mut i = 0;
        let file = File::open(&files[0])?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if i >= count {
                break;
            }
            let line = line?;
            println!("{}", line);
            i += 1;
        }
    }
    Ok(())
}
