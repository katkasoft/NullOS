use std::{fs, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string("/proc/version")?;
    print!("{}", contents);
    Ok(())
}