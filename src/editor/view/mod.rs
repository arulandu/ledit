use crate::editor::terminal::{Terminal, Size, Position};
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    pub size: Size
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
    pub fn render(&mut self) {
        if !self.needs_redraw { return; }
        
        let _ = Terminal::move_cursor_to(Position{col: 0, row: 0});
        for row in 0..self.size.height {
            match self.buffer.lines.get(row) {
                Some(line) => {
                    let truncated = if line.len() >= self.size.width {
                        &line[0..self.size.width]
                    } else {
                        line
                    };

                    Self::render_line(row, truncated);
                }
                None => {
                    // Welcome screen
                    if row == self.size.height / 3 && self.buffer.is_empty() {
                        let mut msg = format!("Welcome to {NAME} -- Version {VERSION}");
                        msg.truncate(self.size.width);
                        let msg_len = msg.len();
                        let padding = if self.size.width > msg_len + 2 { (self.size.width - msg_len)/2-1 } else { 0 };
                        let spaces = " ".repeat(padding as usize);
                        Self::render_line(row, &format!("~{spaces}{msg}"));
                    } else {
                        Self::render_line(row, &format!("~"));
                    }
                }
            }
        } 
        

        self.needs_redraw = false;
    }


    fn render_line(row: usize, txt: &str) {
        let result = Terminal::print_row(row, txt);
        debug_assert!(result.is_ok(), "Failed to render line");
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