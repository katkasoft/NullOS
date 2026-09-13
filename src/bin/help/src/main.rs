use std::{env, fs, io};

fn main() -> io::Result<()> {
    let command = env::args().nth(1).unwrap_or("help".to_string());
    let filename = format!("/etc/help/{}.txt", command);
    let contents = fs::read_to_string(filename)
        .expect("Error when reading help file");
    println!("{}", contents);
    Ok(())
}