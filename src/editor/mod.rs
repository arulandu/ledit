use std::env::args;
use std::io::Error;
use std::panic::{take_hook, set_hook};
use crossterm::event::{read, Event};
mod terminal;
use terminal::{Terminal};
mod view;
use view::{View};
mod editorcommand;
use editorcommand::EditorCommand;

pub struct Editor {
    should_quit: bool,
    view: View
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
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
        match EditorCommand::try_from(event) {
            Ok(cmd) => {
                match cmd {
                    EditorCommand::Quit => {
                        self.should_quit = true;
                    }
                    c => self.view.handle_command(c)
                }
            }
            Err(e) => {
                #[cfg(debug_assertions)]
                {
                    panic!("Could not handle command: {e:?}")
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        let _ = self.view.render();
        let _ = Terminal::move_cursor_to(self.view.get_cursor_position());
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
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