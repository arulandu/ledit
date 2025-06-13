#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub line: usize,
    pub index: usize // grapheme index
}

impl Location {
    pub const fn sub(&self, other: &Self) -> Self {
        Self {
            line: self.line.saturating_sub(other.line),
            index: self.index.saturating_sub(other.index)
        }
    }
}
