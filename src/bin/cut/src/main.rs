use std::{fs, io, env};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    if args.len() < 4 {
        eprintln!("Usage: cut [delimiter] [field] [file]");
        std::process::exit(1);
    }
    let delimiter = &args[1];
    let field: usize = args[2].parse().unwrap_or_else(|_| {
        eprintln!("field must be a number");
        std::process::exit(1);
    });
    if field == 0 {
        eprintln!("field must be 1 or greater");
        std::process::exit(1);
    }
    let file = File::open(&args[3])?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(delimiter).collect();
        if let Some(value) = parts.get(field - 1) {
            println!("{}", value);
        }
    }
    Ok(())
}
