use crate::editor::terminal::{Terminal, Size};
use crate::editor::terminal::position::Position;
mod buffer;
use buffer::Buffer;
use crate::editor::editorcommand::{EditorCommand, Direction};
mod location;
use location::Location;
mod line;
use line::Line;
use log::{debug, info, warn};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location, // text location
    scroll_offset: Position
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Position::default()
        }
    }
}

impl View {
    pub fn handle_command(&mut self, cmd: EditorCommand) {
        debug!("Handling command: {:?}", cmd);
        match cmd {
            EditorCommand::MoveCursor(dir) => self.move_location(dir),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {}
        }
        debug!("Text location:\t{:?}\tScroll offset:\t{:?}", self.location, self.scroll_offset);
    }

    pub fn render(&mut self) {
        if !self.needs_redraw { return; }
        
        let _ = Terminal::move_cursor_to(Position{col: 0, row: 0});
        for row in 0..self.size.height {
            let line_num = row + self.scroll_offset.row;
            match self.buffer.lines.get(line_num) {
                Some(line) => {
                    let left = self.scroll_offset.col;
                    let right = self.scroll_offset.col.saturating_add(self.size.width);
                    let right = std::cmp::min(right, line.len());
                    let truncated = line.get(left..right);
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

    fn resize(&mut self, size: Size) {
        self.size = size;
        self.scroll_location_to_view();
        self.needs_redraw = true;
    }

    fn move_location(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::PageUp => self.move_up(self.size.height.saturating_sub(1)),
            Direction::PageDown => self.move_down(self.size.height.saturating_sub(1)),
            Direction::Home => self.move_to_sol(),
            Direction::End => self.move_to_eol(),
            _ => {}
        }
        
        self.scroll_location_to_view();
    }

    fn move_up(&mut self, delta: usize) {
        self.location.line = self.location.line.saturating_sub(delta);
        self.snap_cursor_to_grapheme();
    }

    fn move_down(&mut self, delta: usize) {
        self.location.line = self.location.line.saturating_add(delta);
        self.snap_cursor_to_grapheme();
        self.snap_cursor_to_line();
    }

    fn move_left(&mut self) {
        if self.location.index > 0 {
            self.location.index = self.location.index.saturating_sub(1);
        } else {
            self.move_up(1);
            self.move_to_eol();
        }
    }

    fn move_right(&mut self) {
        let line_width = self.buffer.lines.get(self.location.line).map_or(0, Line::len);
        if self.location.index < line_width {
            self.location.index = self.location.index.saturating_add(1);
        } else {
            self.move_down(1);
            self.move_to_sol();
        }
    }

    fn move_to_eol(&mut self) {
        self.location.index = self.buffer.lines.get(self.location.line).map_or(0, Line::len);
    }
    
    fn move_to_sol(&mut self) {
        self.location.index = 0;
    }
    

    fn snap_cursor_to_grapheme(&mut self) {
        self.location.index = self.buffer.lines.get(self.location.line).map_or(0, |l| std::cmp::min(l.len(), self.location.index));
    }

    fn snap_cursor_to_line(&mut self) {
        self.location.line = std::cmp::min(self.location.line, self.buffer.line_count().saturating_sub(1));
    }

    fn scroll_line_to_view(&mut self, to: usize) {
        let Size {height, ..} = self.size;
        if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            self.needs_redraw = true;
        } else if to >= self.scroll_offset.row + height {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            self.needs_redraw = true;
        }
    }

    fn scroll_col_to_view(&mut self, to: usize) {
        let Size {width, ..} = self.size;
        if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            self.needs_redraw = true;
        } else if to >= self.scroll_offset.col + width {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            self.needs_redraw = true;
        }
    }

    fn location_to_position(&self) -> Position {
        let row = self.location.line;
        let col = self.buffer.lines.get(row).map_or(0, |line| {
            line.width(0..self.location.index)
        });
        Position {col, row}
    }

    fn scroll_location_to_view(&mut self) {
        let Position {row, col} = self.location_to_position();
        self.scroll_line_to_view(row);
        self.scroll_col_to_view(col);
    }

    pub fn get_cursor_position(&self) -> Position {
        self.location_to_position().saturating_sub(&self.scroll_offset)
    }

    pub fn load_file(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let contents = std::fs::read_to_string(filename)?;
        for line in contents.lines() {
            self.buffer.lines.push(line.into());
            self.needs_redraw = true;
        }
        Ok(())
    }
}