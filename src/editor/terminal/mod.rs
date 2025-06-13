use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;
use crossterm::{queue, Command};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::{stdout, Error, Write};
pub mod position;
use position::Position;

#[derive(Copy, Clone, Default, Debug)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::exit_alternate_screen()?;
        Self::show_cursor()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(pos.col as u16, pos.row as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size{height: height as usize, width: width as usize})
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)
    }

    pub fn exit_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)
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

    pub fn print_row(row: usize, text: &str) -> Result<(), Error> {
        Self::move_cursor_to(Position{col: 0, row})?;
        Self::clear_line()?;
        Self::print(text)?;
        Ok(())
    }

    pub fn queue_command<T:Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }
}