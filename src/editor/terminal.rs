use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;
use crossterm::{queue, Command};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, size};
use std::io::{stdout, Error, Write};


#[derive(Copy, Clone, Default)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }
    
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(pos.x as u16, pos.y as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size{height: height as usize, width: width as usize})
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    pub fn print(text: &str) -> Result<(), Error> {
        Self::queue_command(Print(text))
    }

    pub fn queue_command<T:Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }
}