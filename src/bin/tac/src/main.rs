use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::fs;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
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