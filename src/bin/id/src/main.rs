use std::{fs, io};
use rustix::process::{getuid, getgid};

fn name_from_passwd(uid: u32) -> String {
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

fn name_from_group(gid: u32) -> String {
    if let Ok(contents) = fs::read_to_string("/etc/group") {
        for line in contents.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                if let Ok(line_gid) = parts[2].parse::<u32>() {
                    if line_gid == gid {
                        return parts[0].to_string();
                    }
                }
            }
        }
    }
    gid.to_string()
}

fn main() -> io::Result<()> {
    let uid = getuid().as_raw();
    let gid = getgid().as_raw();
    println!(
        "uid={}({}) gid={}({}) groups={}({})",
        uid,
        name_from_passwd(uid),
        gid,
        name_from_group(gid),
        gid,
        name_from_group(gid)
    );
    Ok(())
}