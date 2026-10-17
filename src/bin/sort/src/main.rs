use std::{env, fs::{self, File}, io::{self, BufRead, BufReader}};

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
    let accepted_params: Vec<String> = vec!["r".to_string()];
    let mut params: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    if args.len() == 1 {
        eprintln!("Usage: ");
        std::process::exit(1);
    }
    if files.is_empty() {
        eprintln!("sort: missing file operand");
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
    let file = File::open(&files[0])?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    if params.contains(&"r".to_string()) {
        for line in lines.into_iter().rev() {
            println!("{}", line);
        }
    } else {
        lines.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        for line in lines {
            println!("{}", line);
        }
    }
    Ok(())
}
