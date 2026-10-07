use std::env;
use std::io;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: find [dir] [name]");
        std::process::exit(1);
    }
    let dir = &args[0];
    for entry in WalkDir::new(dir) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if entry.file_type().is_file() {
            if args.len() == 1 {
                println!("{}", path.display());
            } else {
                if path.file_name().map_or(false, |n| n.to_string_lossy() == args[1]) {
                    println!("{}", path.display());
                }
            }
        }
    }
    Ok(())
}