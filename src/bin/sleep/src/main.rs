use std::{thread, time::Duration};
use std::{io, env, fs};

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
    if args.len() < 2 {
        eprintln!("Usage: sleep [seconds]");
        std::process::exit(1);
    }
    let duration: u64 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("sleep: invalid number: {}", args[1]);
        std::process::exit(1);
    });
    thread::sleep(Duration::from_secs(duration));
    Ok(())
}
