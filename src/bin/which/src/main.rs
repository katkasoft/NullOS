use std::{env, fs, println};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == "--help" {
        let name = std::path::Path::new(&args[0])
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| args[0].clone());
        let filename = format!("/etc/help/{}.txt", name);
        let contents = fs::read_to_string(&filename).unwrap();
        println!("{}", contents);
        std::process::exit(0)
    }
    let path = env::var("PATH").unwrap_or_else(|_| "/bin:/sbin:/usr/bin".to_string());
    let cmd = &args[1];
    for dir in path.split(':') {
        let full_path = format!("{}/{}", dir, cmd);
        if Path::new(&full_path).exists() {
            println!("{}", full_path);
            std::process::exit(0);
        }
    }
    eprintln!("command not found: {}", cmd);
    std::process::exit(1)
}