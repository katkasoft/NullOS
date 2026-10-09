use std::{fs, io};
use rustix::process::getuid;

fn name_from_passwd(uid: u32) -> String {
    let args: Vec<String> = env::args().collect();
    if let Ok(contents) = fs::read_to_string("/etc/passwd") {
        for line in contents.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                if let Ok(line_uid) = parts[2].parse::<u32>() {
                    if line_uid == uid {
                        return parts[0].to_string();
                    }
                }
            }
        }
    }
    uid.to_string()
}

fn main() -> io::Result<()> {
    let uid = getuid().as_raw();
    let name = name_from_passwd(uid);
    println!("{}", name);
    Ok(())
}