use std::{env, eprintln, fs, io, println};
use std::path::{absolute, Path};
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;
use chrono::{DateTime, Local};

fn get_symbolic_permissions(path: &str) -> std::io::Result<String> {
    let metadata = fs::metadata(path)?;
    let mode = metadata.permissions().mode();
    let file_type = if metadata.is_dir() {
        'd'
    } else if metadata.file_type().is_symlink() {
        'l'
    } else {
        '-'
    };
    let chars = ['r', 'w', 'x'];
    let mut result = String::with_capacity(10);
    result.push(file_type);
    for i in (0..9).rev() {
        if (mode & (1 << i)) != 0 {
            result.push(chars[(2 - i % 3) as usize]);
        } else {
            result.push('-');
        }
    }
    Ok(result)
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} bytes", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
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
    if args.len() < 2 {
        eprintln!("Usage: stat [file/dir]");
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    let metadata = fs::metadata(&args[1])?;
    let full_path = absolute(path)?;
    println!("File: {}", full_path.display());
    let size = format_size(metadata.len());
    println!("Size: {}", size);
    if metadata.file_type().is_file() {
        println!("Type: file");
    } else if metadata.file_type().is_dir() {
        println!("Type: directory");
    } else if metadata.file_type().is_symlink() {
        println!("Type: symlink");
    } else if metadata.file_type().is_block_device() {
        println!("Type: block device");
    } else if metadata.file_type().is_char_device() {
        println!("Type: char device");
    } else if metadata.file_type().is_fifo() {
        println!("Type: FIFO");
    } else if metadata.file_type().is_socket() {
        println!("Type: socket");
    } else {
        println!("Type: unknown");
    }
    let permissions = get_symbolic_permissions(&args[1])?;
    println!("Permissions: {}", permissions);
    if let Ok(modified) = metadata.modified() {
        let dt: DateTime<Local> = modified.into();
        println!("Modified: {}", dt.format("%Y-%m-%d %H:%M:%S"));
    }
    Ok(())
}
