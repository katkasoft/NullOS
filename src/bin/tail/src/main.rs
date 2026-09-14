use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let accepted_params: Vec<String> = vec!["n".to_string()];
    let mut params: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    if args.len() == 1 {
        eprintln!("Usage: tail [files] -n [lines]");
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
            let lines = io::BufReader::new(&file_opened).lines().count();
            let lines_skip = lines.saturating_sub(count);
            let mut i = 0;
            let reader = io::BufReader::new(&file_opened);
            for line in reader.lines() {
                i += 1;
                if i <= lines_skip {
                    continue;
                }
                let line = line?;
                println!("{}", line);
            }
        }
    } else if params.contains(&"n".to_string()) {
        if files.len() != 2 {
            eprintln!("Usage: tail [files] -n [lines]");
            std::process::exit(1);
        }
        let count: usize = files[1].parse().unwrap();
        let file_opened = File::open(&files[0])?;
        let lines = io::BufReader::new(&file_opened).lines().count();
        let lines_skip = lines.saturating_sub(count);
        let mut i = 0;
        let reader = io::BufReader::new(&file_opened);
        for line in reader.lines() {
            i += 1;
            if i <= lines_skip {
                continue;
            }
            let line = line?;
            println!("{}", line);
        }
    }
    Ok(())
}