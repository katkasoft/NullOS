use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::process::Command;
use std::path::Path;
use std::{env, print, println};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result as RlResult};

struct RshHelper;

impl Helper for RshHelper {}
impl Hinter for RshHelper {
    type Hint = String;
}
impl Highlighter for RshHelper {}
impl Validator for RshHelper {}

impl Completer for RshHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> RlResult<(usize, Vec<Pair>)> {
        let before_cursor = &line[..pos];
        let word_start = before_cursor.rfind(' ').map(|i| i + 1).unwrap_or(0);
        let word = &before_cursor[word_start..];

        let mut candidates = Vec::new();

        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.starts_with(word) {
                    let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                    let replacement = if is_dir {
                        format!("{}/", name)
                    } else {
                        name.clone()
                    };
                    candidates.push(Pair {
                        display: name,
                        replacement,
                    });
                }
            }
        }

        candidates.sort_by(|a, b| a.display.cmp(&b.display));
        Ok((word_start, candidates))
    }
}

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
        if Path::new("/etc/rshrc").exists() {
            let _ = run_script("/etc/rshrc");
        }
        if let Ok(home) = env::var("HOME") {
            let path = format!("{}/.rshrc", home);
            if Path::new(&path).exists() {
                let _ = run_script(&path);
            }
        }
        let mut rl: Editor<RshHelper, DefaultHistory> = Editor::new().unwrap();
        rl.set_helper(Some(RshHelper));
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