use std::{env, fs, io, println};
use sysinfo::Disks;

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
        let contents = fs::read_to_string(&filename)?;
        println!("{}", contents);
        std::process::exit(0)
    }
    let disks = Disks::new_with_refreshed_list();
    println!("{:<15} {:>6} {:>6} {:>6} {:>4} {}", 
        "Filesystem", "Size", "Used", "Avail", "Use%", "Mounted on");
    for disk in &disks {
        let filesystem = disk.file_system();
        let total_space = disk.total_space();
        let size = format_size(total_space);
        let available_space = disk.available_space();
        let available = format_size(available_space);
        let used_bytes = total_space - available_space;
        let used = format_size(used_bytes);
        let use_per = if total_space > 0 {
            (used_bytes * 100 / total_space) as u32
        } else {
            0
        };
        let mount_point = disk.mount_point();
        println!("{:<15} {:>6} {:>6} {:>6} {:>3}% {}", filesystem.display(), size, used, available, use_per, mount_point.display());
    }
    Ok(())
}
