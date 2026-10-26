use std::{fs, io, env};
   
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
    if args.len() < 3 {
        eprintln!("Usage: ln [target] [link]");
        std::process::exit(1);
    }
    let target = &args[1];
    let link = &args[2];
    std::os::unix::fs::symlink(target, link)?;
    Ok(())
}
