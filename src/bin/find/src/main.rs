use std::env;
use std::io;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() >= 2 && args[1] == "--help" {
        let name = std::path::Path::new(&args[0])
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| args[0].clone());
        let filename = format!("/etc/help/{}.txt", name);
        let contents = fs:read_to_string(&filename)?;
        println!("{}", contents);
        std::process::exit(0)
    }
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