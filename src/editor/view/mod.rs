use crate::editor::terminal::{Terminal, Size, Position};
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default()
        }
    }
}

impl View {
    pub fn render(&self) -> Result<(), std::io::Error> {
        if !self.needs_redraw {return Ok(());}
        


        Terminal::move_cursor_to(Position{x: 0, y: 0})?;
        for row in 0..self.size.height {
            Terminal::clear_line()?;
            match self.buffer.lines.get(row) {
                Some(line) => {
                    let truncated = if line.len() >= self.size.width {
                        &line[0..self.size.width]
                    } else {
                        line
                    };

                    Terminal::print(truncated)?;
                }
                None => {
                    // Welcome screen
                    if row == self.size.height / 3 && self.buffer.is_empty() {
                        let mut msg = format!("Welcome to {NAME} -- Version {VERSION}");
                        msg.truncate(   self.size.width);
                        let msg_len = msg.len();
                        let padding = if self.size.width > msg_len + 2 { (self.size.width - msg_len)/2-1 } else { 0 };
                        let spaces = " ".repeat(padding as usize);
                        Terminal::print(&format!("~{spaces}{msg}"))?;
                    } else {
                        Terminal::print("~")?;
                    }
                }
            }
                  
            if row < self.size.height - 1 {
                Terminal::print("\r\n")?;
            }
        } 
        Ok(())
    }


    pub fn resize(&mut self, size: Size) {
        self.size = size; 
        self.needs_redraw = true;
    }

    pub fn load_file(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let contents = std::fs::read_to_string(filename)?;
        for line in contents.lines() {
            self.buffer.lines.push(line.to_string());
            self.needs_redraw = true;
        }
        Ok(())
    }
}