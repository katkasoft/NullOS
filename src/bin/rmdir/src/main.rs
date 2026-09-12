use std::{io, env, fs};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let accepted_params: Vec<String> = vec!["p".to_string()];
    let mut params: Vec<String> = vec![];
    let mut dirs: Vec<String> = vec![];
    if args.len() < 2 {
        eprintln!("Usage: rmdir [dir(s)] [params]");
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
            dirs.push(arg.clone());
        }
    }
    if params.contains(&"p".to_string()) {
        for dir in dirs.iter() {
            let path = Path::new(dir);
                for ancestor in path.ancestors() {
                    if ancestor.as_os_str().is_empty() {
                        continue;
                    }
                    if let Err(e) = fs::remove_dir(ancestor) {
                        eprintln!("Stopping: dir {:?} is unable to delete ({})", ancestor, e);
                        break; 
                    }
                }

        }
    } else {
        for dir in dirs.iter() {
            fs::remove_dir(dir)?;
        }
    }
    Ok(())
}
