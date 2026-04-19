use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let accepted_params: Vec<String> = vec!["p".to_string()];
    let mut params: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    if args.len() == 1 {
        eprintln!("Usage: mkdir [folders] [params]");
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
    for folder in files {
        let path = Path::new(&folder);
        if params.contains(&"p".to_string()) {
            fs::create_dir_all(path)?;
        } else {
            fs::create_dir(path)?;
        }
    }
    Ok(())
}
