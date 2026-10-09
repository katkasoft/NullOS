use std::{fs, io, env};
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
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == "--help" {
        let name = std::path::Path::new(&args[0])
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| args[0].clone());
        let filename = format!("/etc/help/{}.txt", name);
        let contents = fs:read_to_string(&filename)?;
        println!("{}", contents);
        std::process::exit(0)
    }
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