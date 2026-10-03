use rustyline::DefaultEditor;
use std::process::Command;
use std::path::Path;
use std::{env, print, println};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

fn builtin_clear() {
    println!("\x1B[2J\x1b[1;1H");
}

fn builtin_echo(args: &[&str]) {
    for arg in args.iter() {
        print!("{} ", arg);
    }
    println!();
}

fn run_line(input: &str) {
    let input = input.trim();
    if input.is_empty() || input.starts_with('#') {
        return;
    }
    let mut parts = input.split_whitespace();
    let cmd = match parts.next() {
        Some(c) => c,
        None => return,
    };
    let args: Vec<&str> = parts.collect();
    match cmd {
        "cd" => builtin_cd(&args),
        "pwd" => builtin_pwd(),
        "clear" => builtin_clear(),
        "echo" => builtin_echo(&args),
        "exit" => std::process::exit(0),
        _ => run_command(cmd, &args),
    }
}

fn run_script(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        run_line(&line);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect(); 
    if args.is_empty() {
        let mut rl = DefaultEditor::new().unwrap();
        loop {
            let prompt = match env::current_dir() {
                Ok(path) => {
                    format!("{}$ ", path.display())
                }
                Err(_) => {
                    "$ ".to_string()
                }
            };
            let line = rl.readline(&prompt);
            match line {
                Ok(input) => {
                    let _ = rl.add_history_entry(input.as_str());
                    run_line(&input);
                }
                Err(_) => break,
            }
        }
    } else if args[0] == "-c" {
        if args.len() < 2 {
            eprintln!("rsh: -c requires an argument");
            std::process::exit(1);
        }
        run_line(&args[1]);
    } else {
        for arg in args.iter() {
            if let Err(e) = run_script(arg) {
                eprintln!("rsh: {}: {}", arg, e);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}