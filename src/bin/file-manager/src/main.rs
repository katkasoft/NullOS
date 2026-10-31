use std::{fs, io, env};
use std::path::Path;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::style::Stylize;
use std::io::Write;
use crossterm::terminal;
use std::fs::File;

struct FileEntry {
    name: String,
    is_dir: bool,
}

pub fn delete(path: &str, is_dir: bool) -> io::Result<()> {
    if is_dir {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn read_dir(path: &str) -> io::Result<Vec<FileEntry>> {
    let mut vec= Vec::new();
    let folder = Path::new(path);
    if folder.exists() {
        if let Ok(entries) = fs::read_dir(folder) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                let is_dir = entry.file_type()?.is_dir();
                let fileentry = FileEntry {
                    name: name,
                    is_dir: is_dir
                };
                vec.push(fileentry);
            }
        }
    }
    vec.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.cmp(&b.name)
        }
    });
    Ok(vec)
}

fn input(prompt: &str) -> io::Result<String> {
    disable_raw_mode()?;
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    enable_raw_mode()?;
Ok(input.trim().to_string()) 
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
    let current_dir = env::current_dir()?;
    let mut path = current_dir.to_string_lossy().into_owned();
    let mut entries = read_dir(&path)?;
    enable_raw_mode()?;
    let mut selected: usize = 0;
    let mut offset = 0;
    let mut pending_delete: Option<usize> = None;
    loop {
        let (width, height) = terminal::size()?;
        let max_visible_items = height.saturating_sub(3) as usize;
        execute!(std::io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All), MoveTo(0, 0))?;
        let header_text = format!("  NullOS File manager: {}  ", path);
        let padded_header = format!("{:<width$}", header_text, width = width as usize);
        print!("{}\r\n", padded_header.black().on_white().bold());
        if selected < offset {
            offset = selected;
        }
        if selected >= offset + max_visible_items {
            offset = selected - max_visible_items + 1;
        }
        let end = (offset + max_visible_items).min(entries.len());
        for index in offset..end {
            let item = &entries[index];
            if index == selected {
                if item.is_dir {
                    print!("{}\r\n", format!("> {}. {}/", index, item.name).red().bold());
                } else {
                    print!("{}\r\n", format!("> {}. {}", index, item.name).red().bold());
                }
            } else if item.is_dir {
                print!("{}\r\n", format!("  {}. {}/", index, item.name).blue().bold());
            } else {
                print!("  {}. {}\r\n", index, item.name);
            }
        }
        execute!(std::io::stdout(), MoveTo(0, height - 2))?;
        let footer_text = if let Some(idx) = pending_delete {
            format!(" Delete '{}'? [y/N] ", entries[idx].name)
        } else {
            " [q] Quit  [Enter] Open  [Backspace] Up  [d] Delete [n] New file".to_string()
        };
        let padded_footer = format!("{:<width$}", footer_text, width = width as usize);
        print!("{}", padded_footer.black().on_white().bold());
        let footer2_text = " [m] New directory [r] Rename/Move to [c] Copy file to [s] Stats";
        let padded_footer2 = format!("{:<width$}", footer2_text, width = width as usize);
        print!("{}", padded_footer2.black().on_white().bold());
        std::io::stdout().flush()?;
        if let Event::Key(key_event) = event::read()? {
            match key_event.code { 
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                },
                KeyCode::Down => {
                    if !entries.is_empty() && selected < entries.len() - 1 {
                        selected += 1;
                    }
                },
                KeyCode::Enter | KeyCode::Right => {
                    if !entries.is_empty() && entries[selected].is_dir {
                        let new_path = if path == "/" {
                            format!("/{}", entries[selected].name)
                        } else {
                            format!("{}/{}", path, entries[selected].name)
                        };
                        entries = read_dir(&new_path)?;
                        path = new_path;
                        selected = 0;
                        offset = 0;
                    }
                }
                KeyCode::Backspace | KeyCode::Left => {
                    let p = Path::new(&path);
                    if let Some(parent) = p.parent() {
                        path = parent.to_string_lossy().into_owned();
                        entries = read_dir(&path)?;
                        selected = 0;
                        offset = 0;
                    }
                },
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let target_index = (c as u8 - b'0') as usize;
                    if target_index < entries.len() {
                        selected = target_index;
                    }
                },
                KeyCode::Delete | KeyCode::Char('d') => {
                    if !entries.is_empty() {
                        pending_delete = Some(selected);
                    }
                }
                KeyCode::Char('n') => {
                    let name = input("Enter file name: ")?;
                    if !name.is_empty() {
                        let new_path = if path == "/" {
                            format!("/{}", name)
                        } else {
                            format!("{}/{}", path, name)
                        };
                        let _ = File::create(new_path);
                        entries = read_dir(&path)?;
                    }
                }
                KeyCode::Char('m') => {
                    let name = input("Enter dir name: ")?;
                    if !name.is_empty() {
                        let new_path = if path == "/" {
                            format!("/{}", name)
                        } else {
                            format!("{}/{}", path, name)
                        };
                        let _ = fs::create_dir_all(new_path);
                        entries = read_dir(&path)?;
                    }
                }
                KeyCode::Char('r') => {
                    let name = input("Enter destination name: ")?;
                    if !name.is_empty() && !entries.is_empty() {
                        let old_path = if path == "/" {
                            format!("/{}", entries[selected].name)
                        } else {
                            format!("{}/{}", path, entries[selected].name)
                        };
                        let new_path = if name.starts_with("/") {
                            name
                        } else { 
                            if path == "/" {
                                format!("/{}", name)
                            } else {
                                format!("{}/{}", path, name)
                            }
                        };
                        let _ = fs::rename(old_path, new_path);
                        entries = read_dir(&path)?;
                    }
                }
                KeyCode::Char('c') => {
                    let name = input("Enter destination name: ")?;
                    if !name.is_empty() && !entries.is_empty() {
                        let old_path = if path == "/" {
                            format!("/{}", entries[selected].name)
                        } else {
                            format!("{}/{}", path, entries[selected].name)
                        };
                        let new_path = if name.starts_with("/") {
                            name
                        } else { 
                            if path == "/" {
                                format!("/{}", name)
                            } else {
                                format!("{}/{}", path, name)
                            }
                        };
                        if entries[selected].is_dir {
                            let _ = std::process::Command::new("/bin/cp")
                                .arg("-r")
                                .arg(&old_path)
                                .arg(&new_path)
                                .status();
                        } else {
                            let _ = fs::copy(&old_path, &new_path);
                        }
                        entries = read_dir(&path)?;
                    }
                }
                KeyCode::Char('s') => {
                    if !entries.is_empty() {
                        disable_raw_mode()?;
                        execute!(std::io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All), MoveTo(0, 0))?;
                        std::io::stdout().flush()?;
                        let full_path = if path == "/" {
                            format!("/{}", entries[selected].name)
                        } else {
                            format!("{}/{}", path, entries[selected].name)
                        };
                        let _ = std::process::Command::new("/bin/stat").arg(&full_path).status();
                        print!("\r\nPress Enter to return to menu...");
                        std::io::stdout().flush()?;
                        let mut dummy = String::new();
                        let _ = std::io::stdin().read_line(&mut dummy);
                        enable_raw_mode()?;
                    }
                }
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if let Some(index) = pending_delete {
                        let new_path = if path == "/" {
                            format!("/{}", entries[index].name)
                        } else {
                            format!("{}/{}", path, entries[index].name)
                        };
                        if delete(&new_path, entries[index].is_dir).is_ok() {
                            entries = read_dir(&path)?;
                            if selected >= entries.len() && selected > 0 {
                                selected -= 1;
                            }
                        }
                    }
                    pending_delete = None;
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    break;
                },
                _ => {
                    pending_delete = None;
                }
            }
        }
    }
    disable_raw_mode()?;
    print!("\x1B[2J\x1B[1;1H"); 
    Ok(())
}
