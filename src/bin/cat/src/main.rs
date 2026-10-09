use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
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
    if args.len() == 1 {
        eprintln!("Usage: cat [file]");
        return Ok(());
    }
    for arg in args.iter().skip(1) {
        let file_path = Path::new(arg);
        if !file_path.is_file() {
            eprintln!("File not found: {}", arg);
            continue;
        }
        let file = File::open(arg)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            println!("{}", line);
        }
    }
    Ok(())
}
