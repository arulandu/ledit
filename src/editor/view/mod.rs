use crate::editor::terminal::{Terminal, Size, Position};
mod buffer;
use buffer::Buffer;
use crate::editor::editorcommand::{EditorCommand, Direction};
mod location;
use location::Location;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default()
        }
    }
}

impl View {
    pub fn render(&mut self) {
        if !self.needs_redraw { return; }
        
        let _ = Terminal::move_cursor_to(Position{col: 0, row: 0});
        for row in 0..self.size.height {
            let line_num = row + self.scroll_offset.line;
            match self.buffer.lines.get(line_num) {
                Some(line) => {
                    let left = self.scroll_offset.column;
                    let right = std::cmp::min(left + self.size.width, line.len());
                    let truncated = line.get(left..right).unwrap_or_default();
                    Self::render_line(row, &truncated);
                }
                None => {
                    // Welcome screen
                    if line_num == self.size.height / 3 && self.buffer.is_empty() {
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

    pub fn handle_command(&mut self, cmd: EditorCommand) {
        match cmd {
            EditorCommand::MoveCursor(dir) => {
                match dir {
                    Direction::Up => {
                        if self.scroll_offset.line > 0 && self.location.line == self.scroll_offset.line {
                            self.scroll_offset.line = self.scroll_offset.line.saturating_sub(1);
                            self.needs_redraw = true;
                        }

                        self.location.line = self.location.line.saturating_sub(1);
                    }
                    Direction::Down => {
                        if self.location.line == self.size.height + self.scroll_offset.line - 1 {
                            self.scroll_offset.line = self.scroll_offset.line.saturating_add(1);
                            self.needs_redraw = true;
                        }

                        self.location.line = self.location.line.saturating_add(1);

                    }
                    Direction::Left => {
                        if self.scroll_offset.column > 0 && self.location.column == self.scroll_offset.column {
                            self.scroll_offset.column = self.scroll_offset.column.saturating_sub(1);
                            self.needs_redraw = true;
                        }

                        self.location.column = self.location.column.saturating_sub(1);
                    }
                    Direction::Right => {
                        if self.location.column == self.size.width + self.scroll_offset.column - 1 {
                            self.scroll_offset.column = self.scroll_offset.column.saturating_add(1);
                            self.needs_redraw = true;
                        }

                        self.location.column = self.location.column.saturating_add(1);
                    }
                    Direction::PageUp => {
                        self.location.line = self.scroll_offset.line;
                    }
                    Direction::PageDown => {
                        self.location.line = self.scroll_offset.line + self.size.height - 1;
                    }
                    Direction::Home => {
                        self.location.column = self.scroll_offset.column;
                    }
                    Direction::End => {
                        self.location.column = self.scroll_offset.column + self.size.width - 1;
                    }
                }
            }
            EditorCommand::Resize(size) => {
                self.resize(size);
            }
            _ => {}
        }
    }

    pub fn get_position(&self) -> Position {
        self.location.sub(&self.scroll_offset).into()
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