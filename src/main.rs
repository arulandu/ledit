#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]

mod editor;
mod logger;
use editor::Editor;
use log::info;

fn main() {
    logger::init_logger();
    info!("Starting Ledit editor");
    
    let mut editor = Editor::new().unwrap();
    editor.run();
}
