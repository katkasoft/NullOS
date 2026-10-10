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
use std::collections::HashMap;
use std::fs;

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

struct Shell {
    vars: HashMap<String, String>,
}

impl Shell {
    fn new() -> Self {
        let mut vars = HashMap::new();
        for (k, v) in env::vars() {
            vars.insert(k, v);
        }
        if !vars.contains_key("PATH") {
            vars.insert("PATH".to_string(), "/bin:/sbin:/usr/bin".to_string());
        }
        Shell { vars }
    }
}

fn builtin_export(shell: &mut Shell, args: &[String]) {
    if args.is_empty() {
        for (k, v) in &shell.vars {
            println!("{}={}", k, v);
        }
        return
    }
    for arg in args {
        if let Some((key, value)) = arg.split_once('=') {
            shell.vars.insert(key.to_string(), value.to_string());
        } else {
            eprintln!("export: invalid format: {}", arg);
        }
    }
}

fn builtin_unset(shell: &mut Shell, args: &[String]) {
    for arg in args {
        shell.vars.remove(arg);
    }
}

fn builtin_env(shell: &Shell) {
    for (k, v) in &shell.vars {
        println!("{}={}", k, v);
    }
}

fn run_command(shell: &Shell, cmd: &str, args: &[String]) {
    let path = shell.vars.get("PATH").map(|s| s.as_str()).unwrap_or("/bin:/sbin:/usr/bin");
    if cmd.contains('/') {
        execute(shell, cmd, args);
        return;
    }
    for dir in path.split(':') {
        let full_path = format!("{}/{}", dir, cmd);
        if Path::new(&full_path).exists() {
            execute(shell, &full_path, args);
            return;
        }
    }
    eprintln!("command not found: {}", cmd);
}

fn execute(shell: &Shell, path: &str, args: &[String]) {
    match Command::new(path).args(args).envs(&shell.vars).spawn() {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }
}

fn builtin_cd(shell: &Shell, args: &[String]) {
    let target = if args.is_empty() {
        shell.vars.get("HOME").cloned().unwrap_or_else(|| "/".to_string())
    } else {
        args[0].clone()
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

fn builtin_echo(args: &[String]) {
    for arg in args.iter() {
        print!("{} ", arg);
    }
    println!();
}

fn run_line(shell: &mut Shell, input: &str) {
    let input = input.trim();
    if input.is_empty() || input.starts_with('#') {
        return;
    }
    let mut parts = input.split_whitespace();
    let cmd = match parts.next() {
        Some(c) => c,
        None => return,
    };
    let args: Vec<String> = parts.map(|s| {
        if let Some(key) = s.strip_prefix('$') {
            if let Some(value) = shell.vars.get(key) {
                return value.clone();
            }
        }
        s.to_string()
    }).collect();

    match cmd {
        "cd" => builtin_cd(shell, &args),
        "pwd" => builtin_pwd(),
        "clear" => builtin_clear(),
        "echo" => builtin_echo(&args),
        "env" => builtin_env(shell),
        "export" => builtin_export(shell, &args),
        "unset" => builtin_unset(shell, &args),
        "exit" => std::process::exit(0),
        _ => run_command(shell, cmd, &args),
    }
}

fn run_script(shell: &mut Shell, path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        run_line(shell, &line);
    }
    Ok(())
}

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
    let args: Vec<String> = args.into_iter().skip(1).collect();
    if args.is_empty() {
        let mut shell = Shell::new();
        if Path::new("/etc/rshrc").exists() {
            let _ = run_script(&mut shell, "/etc/rshrc");
        }
        if let Ok(home) = env::var("HOME") {
            let path = format!("{}/.rshrc", home);
            if Path::new(&path).exists() {
                let _ = run_script(&mut shell, &path);
            }
        }
        let mut rl: Editor<RshHelper, DefaultHistory> = Editor::new().unwrap();
        rl.set_helper(Some(RshHelper));
        loop {
            let prompt = match env::current_dir() {
                Ok(path) => format!("{}$ ", path.display()),
                Err(_) => "$ ".to_string(),
            };
            match rl.readline(&prompt) {
                Ok(input) => {
                    let _ = rl.add_history_entry(input.as_str());
                    run_line(&mut shell, &input);
                }
                Err(_) => break,
            }
        }
    } else if args[0] == "-c" {
        if args.len() < 2 {
            eprintln!("rsh: -c requires an argument");
            std::process::exit(1);
        }
        let mut shell = Shell::new();
        run_line(&mut shell, &args[1]);
    } else {
        let mut shell = Shell::new();
        for arg in args.iter() {
            if let Err(e) = run_script(&mut shell, arg) {
                eprintln!("rsh: {}: {}", arg, e);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}