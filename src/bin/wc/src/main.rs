use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
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
    let args = args.into_iter().skip(1).collect();
    if args.len() != 2 {
        eprintln!("Usage: wc [-l|-w|-c|-m] [file]");
        std::process::exit(1);
    }
    let file = File::open(&args[1])?;
    let mut reader = BufReader::new(file);
    let mut count = 0;
    match args[0].as_str() {
        "-l" => {
            for line in reader.lines() {
                let _line = line?;
                count += 1;
            }
        }
        "-w" => {
            for line in reader.lines() {
                let line = line?;
                count += line.split_whitespace().count();
            }
        }
        "-c" => {
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            count = buf.len();
        }
        "-m" => {
            let mut s = String::new();
            reader.read_to_string(&mut s)?;
            count = s.chars().count();
        }
        _ => {
            eprintln!("No such parameter: {}", args[0]);
            std::process::exit(1);
        }
    }
    println!("{}", count);
    Ok(())
}