use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: sysmanager [poweroff|reboot|halt]");
        process::exit(1);
    }

    unsafe { libc::sync(); }

    let cmd = match args[1].as_str() {
        "poweroff" => {
            print!("Powering off system... ");
            io::stdout().flush().unwrap();
            libc::LINUX_REBOOT_CMD_POWER_OFF
        }
        "reboot" => {
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
}
