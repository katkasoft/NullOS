use std::fs::File;
use std::{io, env, fs};
use crossterm::terminal::{self, enable_raw_mode, disable_raw_mode};
use crossterm::style::Stylize;
use crossterm::cursor::MoveTo;
use std::io::{Write, BufWriter};
use crossterm::execute;
use crossterm::event::{self, Event, KeyCode};

fn input(prompt: &str) -> io::Result<String> {
    disable_raw_mode()?;
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    enable_raw_mode()?;
    Ok(input.trim().to_string()) 
}

fn main() -> io::Result<()>{
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
    enable_raw_mode()?;
    let mut lines: Vec<String> = Vec::new();
    let mut filename: Option<String> = if args.len() >= 2 {
        Some(args[1].clone())
    } else {
        None
    };
    if let Some(ref path) = filename {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                lines.push(line.to_string());
            }
        }
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    let mut cursor_x: usize = 0;
    let mut cursor_y: usize = 0;
    let mut offset: usize = 0;
    let mut offset_x: usize = 0;
    let mut pending_quit = false;
    let mut is_modified = false;
    loop {
        let (width, height) = terminal::size()?;
        let max_visible_items = height.saturating_sub(2) as usize;
        execute!(std::io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All), MoveTo(0, 0))?;
        let header_text: String = match filename {
            Some(ref path) => {
                if is_modified {
                    format!(" NullOS text editor - {} [Modified]", path)
                } else {
                    format!(" NullOS text editor - {}", path)
                }
            },
            None => {
                if is_modified {
                    " NullOS text editor [Modified]".to_string()
                } else {
                    " NullOS text editor".to_string()
                }
            }
        };
        let padded_header = format!("{:<width$}", header_text, width = width as usize);
        print!("{}\r\n", padded_header.black().on_white().bold());
        if cursor_y < offset {
            offset = cursor_y;
        }
        if cursor_y >= offset + max_visible_items {
            offset = cursor_y - max_visible_items + 1;
        }
        let end = (offset + max_visible_items).min(lines.len());
        let max_text_width = (width as usize).saturating_sub(1);
        if cursor_x < offset_x {
            offset_x = cursor_x;
        }
        if cursor_x >= offset_x + max_text_width {
            offset_x = cursor_x - max_text_width + 1;
        }
        for y in offset..end {
            let line = &lines[y];
            let visible_line: String = line.chars().skip(offset_x).collect();
            if y == cursor_y {
                let screen_cursor_x = cursor_x.saturating_sub(offset_x);
                let left = &visible_line[..screen_cursor_x.min(visible_line.len())];
                let right = &visible_line[screen_cursor_x.min(visible_line.len())..];
                if let Some(char_under_cursor) = right.chars().next() {
                    let rest = &right[char_under_cursor.len_utf8()..];
                    print!("{}{}{}\r\n", left, char_under_cursor.to_string().black().on_white().bold(), rest);
                } else {
                    print!("{}{}\r\n", left, " ".to_string().black().on_white().bold());
                }
            } else {
                print!("{}\r\n", visible_line);
            }
        }
        execute!(std::io::stdout(), MoveTo(0, height - 1))?;
        let footer_text= if pending_quit {
            "Quit? [y/N]".to_string()
        } else {
            " [^o] Open [^s] Save [^q] Quit".to_string()
        };
        let padded_footer = format!("{:<width$}", footer_text, width = width as usize);
        print!("{}", padded_footer.black().on_white().bold());
        std::io::stdout().flush()?;
        if let Event::Key(key_event) = event::read()? {
            if key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                match key_event.code {
                    KeyCode::Char('q') => {
                        if is_modified {
                            pending_quit = true;
                        } else {
                            break;
                        }
                    },
                    KeyCode::Char('o') => {
                        let name = input("Enter filename: ")?;
                        if !name.is_empty() {
                            if let Ok(content) = fs::read_to_string(&name) {
                                lines.clear();
                                for line in content.lines() {
                                    lines.push(line.to_string());
                                }
                                if lines.is_empty() {
                                    lines.push(String::new());
                                }
                                filename = Some(name);
                                cursor_x = 0;
                                cursor_y = 0;
                                offset = 0;
                                is_modified = false;
                            }
                        }
                    }
                    KeyCode::Char('s') => {
                        if filename.is_none() {
                            let name = input("Enter filename: ")?.trim().to_string();
                            if !name.is_empty() {
                                filename = Some(name);
                            } else {
                                continue;
                            }
                        }
                        if let Some(path) = &filename {
                            if let Ok(file) = File::create(path) {
                                let mut writer = BufWriter::new(file);
                                for line in lines.iter() {
                                    let _ = writeln!(writer, "{}", line);
                                }
                                let _ = writer.flush();
                                is_modified = false;
                            }
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            } else {
                match key_event.code {
                    KeyCode::Up => {
                        if cursor_y > 0 {
                            cursor_y -= 1;
                            if cursor_x > lines[cursor_y].len() {
                                cursor_x = lines[cursor_y].len();
                            }
                        }
                    },
                    KeyCode::Down => {
                        if cursor_y < lines.len() - 1 {
                            cursor_y += 1;
                            if cursor_x > lines[cursor_y].len() {
                                cursor_x = lines[cursor_y].len();
                            }
                        }
                    },
                    KeyCode::Left => {
                        if cursor_x > 0 {
                            cursor_x -= 1;
                        } else {
                            if cursor_y > 0 {
                                cursor_y -= 1;
                                cursor_x = lines[cursor_y].len();
                            }
                        }
                    },
                    KeyCode::Right => {
                        if cursor_x < lines[cursor_y].len() {
                            cursor_x += 1;
                        } else {
                            if cursor_y < lines.len() - 1 {
                                cursor_x = 0;
                                cursor_y += 1;
                            }
                        }
                    },
                    KeyCode::Char(c) => {
                        if pending_quit {
                            if c == 'y' || c == 'Y' {
                                std::process::exit(0)
                            } else {
                                pending_quit = false;
                                continue;
                            }
                        }
                        lines[cursor_y].insert(cursor_x, c);
                        cursor_x += 1;
                        is_modified = true;
                    },
                    KeyCode::Backspace => {
                        if cursor_x > 0 {
                            cursor_x -= 1;
                            lines[cursor_y].remove(cursor_x);
                        } else if cursor_y > 0 {
                            let current_line = lines.remove(cursor_y);
                            cursor_y -= 1;
                            cursor_x = lines[cursor_y].len();
                            lines[cursor_y].push_str(&current_line);
                        }
                        is_modified = true;
                    }
                    KeyCode::Delete => {
                        if cursor_x < lines[cursor_y].len() {
                            lines[cursor_y].remove(cursor_x);
                        } else if cursor_x == lines[cursor_y].len() && cursor_y + 1 < lines.len() {
                            let next_line = lines.remove(cursor_y + 1);
                            lines[cursor_y].push_str(&next_line);
                        }
                        is_modified = true;
                    }
                    KeyCode::Enter => {
                        let right_part = lines[cursor_y].split_off(cursor_x);
                        lines.insert(cursor_y + 1, right_part);
                        cursor_y += 1;
                        cursor_x = 0;
                        is_modified = true;
                    }
                    KeyCode::Esc => {
                        break;
                    },
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    print!("\x1B[2J\x1B[1;1H"); 
    Ok(())
}
