#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}