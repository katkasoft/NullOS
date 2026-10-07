use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: tac [file]");
        std::process::exit(1);
    }
    for arg in &args {
        let file = File::open(arg)?;
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            lines.push(line?);
        }
        for line in lines.iter().rev() {
            println!("{}", line);
        }
    }
    Ok(())
}