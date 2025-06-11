use crate::editor::terminal::Position;

#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub line: usize,
    pub column: usize
}

impl From<Location> for Position {
    fn from(loc: Location) -> Self {
        Self {
            col: loc.column,
            row: loc.line
        }
    }
}

impl Location {
    pub const fn sub(&self, other: &Self) -> Self {
        Self {
            line: self.line.saturating_sub(other.line),
            column: self.column.saturating_sub(other.column)
        }
    }
}
