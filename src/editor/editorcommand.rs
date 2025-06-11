use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::editor::terminal::Size;

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Down,
    Left,
    Right,
}

pub enum EditorCommand {
    MoveCursor(Direction),
    Resize(Size),
    Quit
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent { code, modifiers, .. }) => {
                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                    (KeyCode::Up, _) => Ok(Self::MoveCursor(Direction::Up)),
                    (KeyCode::Down, _) => Ok(Self::MoveCursor(Direction::Down)),
                    (KeyCode::Left, _) => Ok(Self::MoveCursor(Direction::Left)),
                    (KeyCode::Right, _) => Ok(Self::MoveCursor(Direction::Right)),
                    (KeyCode::PageUp, _) => Ok(Self::MoveCursor(Direction::PageUp)),
                    (KeyCode::PageDown, _) => Ok(Self::MoveCursor(Direction::PageDown)),
                    (KeyCode::Home, _) => Ok(Self::MoveCursor(Direction::Home)),
                    (KeyCode::End, _) => Ok(Self::MoveCursor(Direction::End)),
                    _ => Err(format!("KeyCode does not correspond with EditorCommand: {event:?}")),
                }
            }
            Event::Resize(width, height) => {
                Ok(Self::Resize(Size{width: width as usize, height: height as usize}))
            }
            _ => Err(format!("Event does not correspond with EditorCommand: {event:?}")),
        }
    }
}