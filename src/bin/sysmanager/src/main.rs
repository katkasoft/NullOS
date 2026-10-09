use std::io::{self, Write};
use std::process;
use std::fs;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
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
    if args.len() != 2 {
        eprintln!("Usage: sysmanager [poweroff|reboot|halt]");
        process::exit(1);
    }

    unsafe { libc::sync(); }

    let cmd = match args[1].as_str() {
        "poweroff"| "shutdown" => {
            print!("Powering off system... ");
            io::stdout().flush().unwrap();
            libc::LINUX_REBOOT_CMD_POWER_OFF
        }
        "reboot" | "restart" => {
            print!("Rebooting system... ");
            io::stdout().flush().unwrap();
            libc::LINUX_REBOOT_CMD_RESTART
        }
        "halt" => {
            print!("Halting system... ");
            io::stdout().flush().unwrap();
            libc::LINUX_REBOOT_CMD_HALT
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    };

    unsafe {
        let ret = libc::reboot(cmd);
        if ret != 0 {
            eprintln!("\nFailed to {}: {}", args[1], io::Error::last_os_error());
            process::exit(1);
        }
    }
    Ok(())
}
