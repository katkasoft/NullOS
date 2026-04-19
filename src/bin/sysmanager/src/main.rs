use std::io::{self, Write};
use std::process;

const LINUX_REBOOT_MAGIC1: i32 = 0xfee1dead;
const LINUX_REBOOT_MAGIC2: i32 = 0x28121969;
const LINUX_REBOOT_CMD_RESTART: i32 = 0x01234567;
const LINUX_REBOOT_CMD_POWER_OFF: i32 = 0x4321fedc;
const LINUX_REBOOT_CMD_HALT: i32 = 0xcdef0123;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [poweroff|reboot|halt]", args[0]);
        process::exit(1);
    }

    unsafe { libc::sync(); }

    let cmd = match args[1].as_str() {
        "poweroff" => {
            print!("Powering off system... ");
            io::stdout().flush().unwrap();
            LINUX_REBOOT_CMD_POWER_OFF
        }
        "reboot" => {
            print!("Rebooting system... ");
            io::stdout().flush().unwrap();
            LINUX_REBOOT_CMD_RESTART
        }
        "halt" => {
            print!("Halting system... ");
            io::stdout().flush().unwrap();
            LINUX_REBOOT_CMD_HALT
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    };

    unsafe {
        let ret = libc::reboot(
            LINUX_REBOOT_MAGIC1,
            LINUX_REBOOT_MAGIC2,
            cmd,
            std::ptr::null_mut(),
        );
        if ret != 0 {
            eprintln!("\nFailed to {}: {}", args[1], io::Error::last_os_error());
            process::exit(1);
        }
    }
}