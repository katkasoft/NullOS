use std::ffi::CString;
use std::ptr;
use libc::{mount, execv};

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
    println!("Starting shell...");
    let shell = CString::new("/bin/rsh").unwrap();
    let args = [shell.as_ptr(), ptr::null()];
    unsafe {
        execv(shell.as_ptr(), args.as_ptr());
    }
    panic!("exec failed");
}