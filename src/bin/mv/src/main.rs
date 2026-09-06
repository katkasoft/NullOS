use std::{env, fs, io, println};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: mv [source] [destitation]");
        std::process::exit(1);
    }
    let source = args[1].clone();
    let destitation = args[2].clone();
    if destitation.chars().last() == Some('/') {
        let source_path = Path::new(&source);
        let file_name = source_path
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;
        let mut destination = Path::new(&destitation).to_path_buf();
        destination.push(file_name);
        fs::create_dir_all(&source)?;
        fs::rename(source_path, destination)?;
    } else {
        fs::rename(source, destitation)?;
    }
    Ok(())
}
