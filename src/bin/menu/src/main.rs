use std::{fs, io, env};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::style::Stylize;

struct MenuItem {
    label: String,
    action: String,
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
    print!("\x1B[2J\x1B[1;1H");
    let filename = match args.len() {
        1 => "/etc/menu/menu.conf".to_string(),
        _ => format!("/etc/menu/{}.conf", args[1].clone())
    };
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut items: Vec<MenuItem> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() && !line.trim().starts_with("#") {
            if let Some((label, action)) = line.split_once(":") {
                let item = MenuItem {
                    label: label.trim().to_string(),
                    action:action.trim().to_string()
                };
                items.push(item);
            } 
        }
    }
    enable_raw_mode()?;
    let mut selected: usize = 0;
    loop {
        execute!(std::io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All), MoveTo(0, 0))?; 
        print!("=== NullOS Menu ===\r\n");
        print!("\r\n");
        for (index, item) in items.iter().enumerate() {
            if index == selected {
                print!("{}\r\n", format!("> {}. {}", index, item.label).red().bold());
            } else {
                print!("  {}. {}\r\n", index, item.label);
            }
        }
        std::io::stdout().flush()?;
        if let Event::Key(key_event) = event::read()? {
            match key_event.code { 
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                },
                KeyCode::Down => {
                    if selected < items.len() - 1 {
                        selected += 1;
                    }
                },
                KeyCode::Enter => {
                    let action = &items[selected].action;
                    disable_raw_mode()?;
                    execute!(std::io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All), MoveTo(0, 0))?;
                    std::io::stdout().flush()?;
                    let mut parts = action.split_whitespace();
                    if let Some(cmd) = parts.next() {
                        let args: Vec<&str> = parts.collect();
                        match std::process::Command::new(cmd).args(&args).status() {
                            Ok(_) => {}, // Программа завершилась нормально (или сама упала)
                            Err(e) => {
                                eprintln!("Failed to run {}: {}", cmd, e);
                                let _ = std::io::stdin().read_line(&mut String::new());
                            }
                        }
                    }
                    enable_raw_mode()?;
                },
                KeyCode::Esc | KeyCode::Char('q') => {
                    break;
                },
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    print!("\x1B[2J\x1B[1;1H"); 
    Ok(())
}
