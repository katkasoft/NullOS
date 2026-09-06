use std::{env, io, path::Path};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let accepted_params: Vec<String> = vec!["r".to_string(), "f".to_string()];
    let mut params: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    if args.len() == 1 {
        eprintln!("Usage: rm [file/folder] [params]");
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
    for file in files {
        let path = Path::new(&file);
        if !path.exists() {
            eprintln!("{} not exists", file);
            continue;
        }
        if params.contains(&"f".to_string()) {
            if params.contains(&"r".to_string()) {
                 
            }
        }
    }
    Ok(())
}
