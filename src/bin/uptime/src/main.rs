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
    let contents = fs::read_to_string("/proc/uptime")?;
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
                println!("up {}d {}h {}m {}s", days, hours, mins, secs_rem);
            } else if hours > 0 {
                println!("up {}h {}m {}s", hours, mins, secs_rem);
            } else if mins > 0 {
                println!("up {}m {}s", mins, secs_rem);
            } else {
                println!("up {}s", secs_rem);
            }
        },
        None => println!("Failed to get uptime"),
    }
    Ok(())
}