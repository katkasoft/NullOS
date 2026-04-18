use std::env;
use std::fs::File;
use std::io::Read;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
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
        let mut file = File::open(arg)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        println!("{}", contents);
    }

    Ok(())
}
