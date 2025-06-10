use crossterm::event::{read, Event::Key, KeyCode::Char, KeyCode, KeyEvent, KeyEventKind, Event, KeyModifiers};
mod terminal;
use terminal::{Terminal, Size, Position};
mod view;
use view::View;

#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub line: usize,
    pub column: usize
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {break;}
            let event = read()?;
            self.evaluate_event(&event)?;
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {code, modifiers, kind: KeyEventKind::Press, ..}) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up => {
                    self.location.line = self.location.line.saturating_sub(1);
                }
                KeyCode::Down => {
                    self.location.line = self.location.line.saturating_add(1);
                }
                KeyCode::Left => {
                    self.location.column = self.location.column.saturating_sub(1);
                }
                KeyCode::Right => {
                    self.location.column = self.location.column.saturating_add(1);
                }
                KeyCode::PageUp => {
                    self.location.line = 0;
                }
                KeyCode::PageDown => {
                    self.location.line = Terminal::size()?.height.saturating_sub(1);
                }
                KeyCode::Home => {
                    self.location.column = 0;
                }
                KeyCode::End => {
                    self.location.column = Terminal::size()?.width.saturating_sub(1);
                }
                _ => ()
            }
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(Position{x: 0, y: 0})?;
            Terminal::print("Goodbye\r\n")?;
        } else {
            View::render()?; 
            Terminal::move_cursor_to(Self::location_to_position(&self.location))?;
        }

        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }

    fn location_to_position(location: &Location) -> Position {
        Position{x: location.column, y: location.line}
    }


}