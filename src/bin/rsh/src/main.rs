use rustyline::DefaultEditor;
use std::process::Command;
use std::path::Path;
use std::env;

fn run_command(cmd: &str, args: &[&str]) {
    let paths = ["/bin", "/sbin", "/usr/bin"];
    if cmd.contains('/') {
        execute(cmd, args);
        return;
    }
    for dir in &paths {
        let full_path = format!("{}/{}", dir, cmd);
        if Path::new(&full_path).exists() {
            execute(&full_path, args);
            return;
        }
    }
    eprintln!("command not found: {}", cmd);
}

fn execute(path: &str, args: &[&str]) {
    match Command::new(path).args(args).spawn() {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }
}

fn builtin_cd(args: &[&str]) {
    let target = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| "/".to_string())
    } else {
        args[0].to_string()
    };
    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}: {}", target, e);
    }
}

fn builtin_pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd error: {}", e),
    }
}

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let prompt = match env::current_dir() {
            Ok(path) => {
                format!("{}$", path.display())
            }
            Err(_) => {
                "$ ".to_string()
            }
        };
        let line = rl.readline(&prompt);
        match line {
            Ok(input) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }
                if input == "exit" {
                    break;
                }
                let _ = rl.add_history_entry(input);
                let mut parts = input.split_whitespace();
                let cmd = parts.next().unwrap();
                let args: Vec<&str> = parts.collect();
                match cmd {
                    "cd" => builtin_cd(&args),
                    "pwd" => builtin_pwd(),
                    _ => run_command(cmd, &args),
                }
            }
            Err(_) => break,
        }
    }
}