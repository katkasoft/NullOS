use std::{env, fs, io};

fn main() -> io::Result<()> {
    let command = env::args().nth(1).unwrap_or("help".to_string());
    let filename = format!("/etc/help/{}.txt", command);
    let contents = match fs::read_to_string(&filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("help: no manual for '{}'", command);
            std::process::exit(1);
        }
    };
    println!("{}", contents);
    Ok(())
}