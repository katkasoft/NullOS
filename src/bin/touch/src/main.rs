use std::{env, io, println};
use std::path::Path;
use std::fs::{File, FileTimes};
use std::time::SystemTime;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: touch [file(s)]");
        std::process::exit(1);
    }
    for arg in args.iter().skip(1) {
        let path = Path::new(&arg);
        match path.try_exists() {
            Ok(true) => {
                let file = File::options().write(true).open(&arg)?;
                 let now = SystemTime::now();
                let times = FileTimes::new()
                    .set_accessed(now)
                    .set_modified(now);
                file.set_times(times)?;
            },
            Ok(false) => {
                let _ = File::create_new("output.txt")?;
            },
            Err(e) => eprintln!("Failed to check if file exists: {}", e),
        }
    }
    Ok(())
}
