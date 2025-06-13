use crate::editor::view::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}