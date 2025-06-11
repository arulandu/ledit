use crossterm::event::{read, Event::Key, KeyCode::Char, KeyCode, KeyEvent, KeyEventKind, Event, KeyModifiers};
mod terminal;
use terminal::{Terminal, Position, Size};
mod view;
use view::View;
use std::env::args;
use std::io::{Error};
use std::panic::{take_hook, set_hook};

#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub line: usize,
    pub column: usize
}

pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            hook(panic_info);
        }));
        Terminal::initialize()?;

        let mut view = View::default();
        let args: Vec<String> = args().collect();
        if let Some(arg) = args.get(1) {
            view.load_file(arg)?;
        }

        Ok(Self {
            should_quit: false,
            location: Location::default(),
            view
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {break;}
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(e) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Failed to read event: {e:?}")
                    }
                }
            }
        }

    }
    
    fn evaluate_event(&mut self, event: Event) {
        match event {
        Key(KeyEvent {code, modifiers, kind: KeyEventKind::Press, ..}) => {
            match code {
                Char('q') if modifiers == KeyModifiers::CONTROL => {
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
                    self.location.line = self.view.size.height.saturating_sub(1);
                }
                KeyCode::Home => {
                    self.location.column = 0;
                }
                KeyCode::End => {
                    self.location.column = self.view.size.width.saturating_sub(1);
                }
                _ => ()
            }
        }
        Event::Resize(width, height) => {
            self.view.resize(Size{width: width as usize, height: height as usize});
        }
        _ => {}
    }

    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        let _ = self.view.render();
        let _ = Terminal::move_cursor_to(Self::location_to_position(&self.location));
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
    }

    fn location_to_position(location: &Location) -> Position {
        Position{col: location.column, row: location.line}
    }


}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye\r\n");
        }
    }
}