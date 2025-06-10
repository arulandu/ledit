use std::io::Error;
use crate::editor::terminal::{Terminal, Size, Position};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View;

impl View {
    pub fn render() -> Result<(), std::io::Error> {
        let Size{width, height} = Terminal::size()?;

        Terminal::move_cursor_to(Position{x: 0, y: 0})?;
        for row in 0..height {
            Terminal::clear_line()?;        
            match row {
                0 => Terminal::print("Hello, World!")?,
                r if r == height / 3 => {
                    let mut msg = format!("Welcome to {NAME} -- Version {VERSION}");
                    msg.truncate(width as usize);
                    let msg_len = msg.len();
                    let padding = if width > msg_len + 2 { (width - msg_len)/2-1 } else { 0 };
                    let spaces = " ".repeat(padding as usize);
                    Terminal::print(&format!("~{spaces}{msg}"))?;
                },
                _ => Terminal::print("~")?
            }
            if row < height - 1 {
                Terminal::print("\r\n")?;
            }
        } 
        Ok(())
    }
}