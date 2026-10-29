use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    print!("Enter command name:");
    std::io::stdout().flush()?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    if input.is_empty() {
        return Ok(());
    }
    std::process::Command::new("/bin/rsh").arg("-c").arg(input).status()?;
    Ok(())
}
