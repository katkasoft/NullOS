use std::env;
use std::path::Path;
use std::fs;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
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
    let accepted_params= vec!["a".to_string(), "f".to_string()];
    let mut params: Vec<String> = vec![];
    let mut folders: Vec<String> = vec![];
    for arg in args {
        if arg.starts_with('-') {
            for chr in arg.chars() {
                if accepted_params.contains(&chr.to_string()) {
                    params.push(chr.to_string());
                } else {
                    eprintln!("No such parametre: -{}", chr)
                }
            }
        } else {
            let folder = Path::new(&arg);
            if folder.exists() {
                folders.push(arg);
            } else {
                eprintln!("Folder does not exists: {}", arg)
            }
        }
    }
    if folders.is_empty() {
        let path = env::current_dir()?;
        folders.push(format!("{}", path.display()));
    }
    let mut first = true;
    for folder in folders {
        let dir = Path::new(&folder);
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = match params.contains(&"f".to_string()) {
                        true => format!("{}", entry.path().display()),
                        false => entry.path().file_name().unwrap().to_string_lossy().into_owned(),
                    };
                    if !path.starts_with(".") || params.contains(&"a".to_string()) {
                        if params.contains(&"f".to_string()) {
                            println!("{}", path)
                        } else {
                            if !first {
                                print!(" ");
                            }
                            print!("{}", path);
                            first = false;
                        }             
                    }
                }    
            }
            if !params.contains(&"f".to_string()) {
                println!();
            }
        }
    }
    Ok(())
}
