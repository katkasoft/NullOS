use std::{fs, io, env};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::style::Stylize;
use std::io::Write;
use crossterm::terminal;
use rustix::process::{kill_process, Pid, Signal};

struct Process {
    pid: u32,
    cmdline: String,
    memory_mb: u64
}

fn read_processes() -> io::Result<Vec<Process>> {
    let mut vec = Vec::new();
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Ok(pid) = name.parse::<u32>() {
                let cmdline = fs::read(format!("/proc/{}/cmdline", pid))
                    .unwrap_or_default();
                let content = fs::read_to_string(format!("/proc/{}/stat", pid)).unwrap_or_default();
                let cmd = if cmdline.is_empty() {
                    let comm = content
                        .split_once('(')
                        .and_then(|(_, rest)| rest.split_once(')'))
                        .map(|(name, _)| name.to_string())
                        .unwrap_or_else(|| "[unknown]".to_string());
                    format!("[{}]", comm)
                } else {
                    cmdline
                        .split(|&b| b == 0)
                        .filter(|s| !s.is_empty())
                        .map(|s| String::from_utf8_lossy(s).into_owned())
                        .collect::<Vec<_>>()
                        .join(" ")
                };
                let content = fs::read_to_string(format!("/proc/{}/status", pid)).unwrap_or_default();
                let mut memory_mb: u64 = 0;
                for line in content.lines() {
                    if line.starts_with("VmRSS:") {
                        if let Some(val) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = val.parse::<u64>() {
                                memory_mb = kb / 1024;
                            }
                        }
                    }
                }
                let process = Process {
                    pid: pid,
                    cmdline: cmd,
                    memory_mb: memory_mb,
                };
                vec.push(process);
            }
        }
    }
    vec.sort_by(|a, b| a.pid.cmp(&b.pid));
    Ok(vec)
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
    enable_raw_mode()?;
    let mut processes = read_processes()?;
    let mut selected: usize = 0;
    let mut offset = 0;
    loop {
        let (width, height) = terminal::size()?;
        let max_visible_items = height.saturating_sub(3) as usize;
        execute!(std::io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All), MoveTo(0, 0))?;
        let header_text = " NullOS Process Manager".to_string();
        let padded_header = format!("{:<width$}", header_text, width = width as usize);
        print!("{}\r\n", padded_header.black().on_white().bold());
        let header2_text =  format!("  {:<5}  {:<7}  {}", "PID", "MEM", "COMMAND");
        let padded_header2 =format!("{:<width$}", header2_text, width = width as usize);
        print!("{}\r\n", padded_header2.black().on_white().bold());
        if selected < offset {
            offset = selected;
        }
        if selected >= offset + max_visible_items {
            offset = selected - max_visible_items + 1;
        }
        let end = (offset + max_visible_items).min(processes.len());
        for index in offset..end {
            let item = &processes[index];
            let line_str = format!("{:<5} {:>4} MB  {}", item.pid, item.memory_mb, item.cmdline); 
            if index == selected {
                print!("{}\r\n", format!("> {}", line_str).red().bold());
            } else {
                print!("  {}\r\n", line_str);
            }
        }
        execute!(std::io::stdout(), MoveTo(0, height - 1))?;
        let footer_text = "[q] Quit [k] Kill process";
        let padded_footer = format!("{:<width$}", footer_text, width = width as usize);
        print!("{}", padded_footer.black().on_white().bold());
        std::io::stdout().flush()?;
        if event::poll(Duration::from_secs(5))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        } else {
                            selected = processes.len() - 1;
                        }
                    },
                    KeyCode::Down => {
                        if !processes.is_empty() {    
                            if selected < processes.len() - 1 {
                                selected += 1;
                            } else {
                                selected = 0;
                            }
                        }
                    },
                    KeyCode::Delete | KeyCode::Char('k') => {
                        if !processes.is_empty() {
                            let sig: i32 = 15;
                            let raw_pid = processes[selected].pid as i32;
                            let pid = match Pid::from_raw(raw_pid) {
                                Some(p) => p,
                                None => {
                                    eprintln!("Invalid PID.");
                                    std::process::exit(1)
                                }
                            };
                            let sig = match Signal::from_raw(sig) {
                                Some(s) => s,
                                None => {
                                    eprintln!("Invalid numeric signal number: {}", sig);
                                    std::process::exit(1)
                                }
                            };
                            if let Err(e) = kill_process(pid, sig) {
                                eprintln!("Failed to kill process: {}", e)
                            }
                        }
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        processes = read_processes()?;
        if selected >= processes.len() && !processes.is_empty() {
            selected = processes.len() - 1;
        } else if processes.is_empty() {
            selected = 0;
        }
    }
    disable_raw_mode()?;
    print!("\x1B[2J\x1B[1;1H"); 
    Ok(())
}