use std::{env, eprintln, fs, io, println};
use std::path::Path;
use walkdir::WalkDir;

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} bytes", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}


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
    let accepted_params: Vec<String> = vec!["s".to_string()];
    let mut params: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
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
    let dir: String = match args.len() {
        1 => ".".to_string(),
        _ => files[0].clone()
    };
    let path = Path::new(&dir);
    if !path.exists() {
        eprintln!("Path does not exists");
        std::process::exit(1);
    }
    let mut total_size = 0;
    let summary = params.contains(&"s".to_string());
    for entry in WalkDir::new(path) {
        let entry = entry.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let metadata = entry.metadata().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        if metadata.is_file() {
            let len = metadata.len();
            if !summary {
                let len_formatted = format_size(len);
                println!("{:>8}\t{}", len_formatted, entry.path().display());
            }
            total_size += len;
        }
    }
    println!("{:>8}\t{}", format_size(total_size), path.display());
    Ok(())
}
