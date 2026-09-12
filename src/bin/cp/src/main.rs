use std::{io, env, fs};
use fs_extra::dir::CopyOptions;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let accepted_params: Vec<String> = vec!["r".to_string()];
    let mut params: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    if args.len() < 3 {
        eprintln!("Usage: cp [file|dir] [destination] [params]");
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
    if files.len() != 2 {
        println!("Usage: cp [file|dir] [destination] [params]");
        std::process::exit(1);
    }
    if params.contains(&"r".to_string()) {
        let from = files[0].clone();
        let to = files[1].clone();
        let mut opt = CopyOptions::new();
        opt.copy_inside = true;
        fs_extra::dir::copy(from, to, &opt)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    } else {
        let file = files[0].clone();
        let dest = files[1].clone();
        fs::copy(file, dest)?;
    }
    Ok(())
}
