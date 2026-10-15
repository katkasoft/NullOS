use std::{env, fs, io, println};
use std::io::{BufRead, BufReader};
use std::fs::File;

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
    let file = File::open("/proc/meminfo")?;
    let reader = BufReader::new(file);
    let mut memtotal: u64 = 0;
    let mut memavailable: u64 = 0;
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("MemTotal:") {
            let second_word = line.split_whitespace().nth(1).unwrap();
            let memtotal_kb: u64 = second_word.parse().unwrap();
            memtotal = memtotal_kb / 1024;
            println!("MemTotal:     {} MB", memtotal);
        } else if line.starts_with("MemFree:") {
            let second_word = line.split_whitespace().nth(1).unwrap();
            let memfree_kb: u64 = second_word.parse().unwrap();
            let memfree = memfree_kb / 1024;
            println!("MemFree:      {} MB", memfree);
        } else if line.starts_with("MemAvailable:") {
            let second_word = line.split_whitespace().nth(1).unwrap();
            let memavailable_kb: u64 = second_word.parse().unwrap();
            memavailable = memavailable_kb / 1024;
            println!("MemAvailable: {} MB", memavailable);
        }
    }
    let used = memtotal.saturating_sub(memavailable);
    println!("Used:         {} MB", used);
    Ok(())
}