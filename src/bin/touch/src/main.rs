use std::{env, io, println};
use std::path::Path;
use std::fs::{File, FileTimes};
use std::time::SystemTime;

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
