use std::{env, fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use nix::sys::statvfs::statvfs;
use std::path::Path;

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{}", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.0}K", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}M", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1}G", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn get_os() -> String {
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("PRETTY_NAME") {
                    let result = line.replace("PRETTY_NAME=", "");
                    let result = result.replace("\"", "");
                    return result
                }
            }
        }
    }
    "unknown".to_string()
}

fn get_hostname() -> String {
    fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string()
}

fn get_kernel() -> String {
    if let Ok(content) = fs::read_to_string("/proc/version") {
        let words: Vec<&str> = content.split_whitespace().collect();
        if words.len() >= 3 {
            return words[2].to_string()
        }
    }
    "unknown".to_string()
}

fn get_uptime() -> String {
    if let Ok(contents) = fs::read_to_string("/proc/uptime") {
        let word = contents.split_whitespace().next();
        match word {
            Some(uptime_str) => {
                let secs: u64 = uptime_str.parse::<f64>()
                    .map(|f| f as u64)
                    .unwrap_or(0);
                let days = secs / 86400;
                let hours = (secs % 86400) / 3600;
                let mins = (secs % 3600) / 60;
                let secs_rem = secs % 60;
                if days > 0 {
                    return format!("{}d {}h {}m {}s", days, hours, mins, secs_rem);
                } else if hours > 0 {
                    return format!("{}h {}m {}s", hours, mins, secs_rem);
                } else if mins > 0 {
                    return format!("{}m {}s", mins, secs_rem);
                } else {
                    return format!("{}s", secs_rem);
                }
            },
            None => return "unknown".to_string(),
        }
    }
    "unknown".to_string()
}

fn get_cpu() -> String {
    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("model name") {
                    if let Some((_, v)) = line.split_once(':') {
                        return v.trim().to_string();
                    }
                }
            }
        }
    }
    "unknown".to_string()
}

fn get_memory() -> String {
    if let Ok(file) = File::open("/proc/meminfo") {
        let reader = BufReader::new(file);
        let mut memtotal: u64 = 0;
        let mut memavailable: u64 = 0;
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("MemTotal:") {
                    let second_word = line.split_whitespace().nth(1).unwrap();
                    let memtotal_kb: u64 = second_word.parse().unwrap();
                    memtotal = memtotal_kb / 1024;
                } else if line.starts_with("MemAvailable:") {
                    let second_word = line.split_whitespace().nth(1).unwrap();
                    let memavailable_kb: u64 = second_word.parse().unwrap();
                    memavailable = memavailable_kb / 1024;
                }
            }
        }
        let used = memtotal.saturating_sub(memavailable);
        return format!("{}/{} MB", used, memtotal)
    }
    "unknown".to_string()
}

fn get_disk() -> String {
    if let Ok(file) = File::open("/proc/mounts") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let words: Vec<&str> = line.split_whitespace().collect();
                if words.len() < 2 {
                    continue;
                }
                let mount_point = words[1];
                if mount_point == "/" {
                    let mount_path = Path::new(&mount_point);
                    if let Ok(stats) = statvfs(mount_path) {
                        let block_size = stats.fragment_size();
                        let total_size = stats.blocks() * block_size; 
                        let available_size = stats.blocks_available() * block_size;
                        let used_size = total_size - available_size;
                        let total = format_size(total_size);
                        let used = format_size(used_size);
                        return format!("{}/{}", used, total);
                    }
                }
            }
        }
    }
    "unknown".to_string()
}

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
    let accepted_params: Vec<String> = vec![
        "s".to_string(), "h".to_string(), "k".to_string(), 
        "u".to_string(), "c".to_string(), "m".to_string(), 
        "d".to_string()
    ];
    let mut params: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
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
    if params.is_empty() {
        println!("OS: {}", get_os());
        println!("Hostname: {}", get_hostname());
        println!("Kernel: {}", get_kernel());
        println!("Uptime: {}", get_uptime());
        println!("CPU: {}", get_cpu());
        println!("Memory: {}", get_memory());
        println!("Disk (/): {}", get_disk())
    }
    if params.contains(&"s".to_string()) {
        println!("{}", get_os())
    }
    if params.contains(&"h".to_string()) {
        println!("{}", get_hostname())
    }
    if params.contains(&"k".to_string()) {
        println!("{}", get_kernel())
    }
    if params.contains(&"u".to_string()) {
        println!("{}", get_uptime())
    }
    if params.contains(&"c".to_string()) {
        println!("{}", get_cpu())
    }
    if params.contains(&"m".to_string()) {
        println!("{}", get_memory())
    }
    if params.contains(&"d".to_string()) {
        println!("{}", get_disk())
    }
    Ok(())
}
