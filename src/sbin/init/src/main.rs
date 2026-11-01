use std::ffi::CString;
use std::ptr;
use libc::{mount, fork, waitpid, execv, WIFEXITED, WEXITSTATUS};

fn mount_fs(source: &str, target: &str, fstype: &str, flags: libc::c_ulong) {
    let src = CString::new(source).unwrap();
    let tgt = CString::new(target).unwrap();
    let fstype = CString::new(fstype).unwrap();

    unsafe {
        if mount(
            src.as_ptr(),
            tgt.as_ptr(),
            fstype.as_ptr(),
            flags,
            ptr::null(),
        ) != 0 {
            panic!("mount failed for {}", target);
        }
    }
}

fn main() {
    println!("Initialising system...");
    std::fs::create_dir_all("/proc").ok();
    std::fs::create_dir_all("/sys").ok();
    std::fs::create_dir_all("/dev").ok();
    std::fs::create_dir_all("/run").ok();
    println!("Mounting kernel virtual filesystems...");
    mount_fs("proc", "/proc", "proc", 0);
    mount_fs("sysfs", "/sys", "sysfs", 0);
    mount_fs("devtmpfs", "/dev", "devtmpfs", 0);
    mount_fs("tmpfs", "/run", "tmpfs", 0);
    println!("Starting menu...");
    std::env::set_var("HOME", "/root");
    let shell = CString::new("/bin/menu").unwrap();
    let args = [shell.as_ptr(), ptr::null()];
    loop {
        unsafe {
            let pid = fork();
            if pid == 0 {
                execv(shell.as_ptr(), args.as_ptr());
                libc::exit(1); 
            } else if pid > 0 {
                let mut status = 0;
                waitpid(pid, &mut status, 0);
                if WIFEXITED(status) {
                    let exit_code = WEXITSTATUS(status);
                    if exit_code == 0 {
                        libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF);
                    }
                }
            } else {
                panic!("fork failed");
            }
        }
    }
}