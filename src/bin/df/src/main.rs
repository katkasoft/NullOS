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

    fn main() -> io::Result<()> {
        let args: Vec<String> = env::args().collect();
        if args.len() >= 2 && args[1] == "--help" {
            let name = std::path::Path::new(&args[0])
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| args[0].clone());
            let filename = format!("/etc/help/{}.txt", name);
            let contents: String = fs::read_to_string(&filename)?;
            println!("{}", contents);
            std::process::exit(0)
        }
        let file = File::open("/proc/mounts")?;
        let reader = BufReader::new(file);
        println!("{:<15} {:>6} {:>6} {:>6} {:>4} {}", 
            "Filesystem", "Size", "Used", "Avail", "Use%", "Mounted on");
        for line in reader.lines() {
            let line = line?;
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() < 2 {
                continue;
            }
            let device = words[0];
            let mount_point = words[1];
            let mount_path = Path::new(&mount_point);
            let stats = statvfs(mount_path)?;

            let block_size = stats.fragment_size(); 
            let total_size = stats.blocks() * block_size; 
            let available_size = stats.blocks_available() * block_size;
            let used_size = total_size - available_size;
            let use_percent = if total_size > 0 {
                used_size * 100 / total_size
            } else {
                0
            };

            let total = format_size(total_size);
            let used = format_size(used_size);
            let avail = format_size(available_size);

            println!("{:<15} {:>6} {:>6} {:>6} {:>3}% {}", device, total, used, avail, use_percent, mount_point);
        }
        Ok(())
    }
