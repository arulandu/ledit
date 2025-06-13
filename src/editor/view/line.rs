use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;
use std::ops::Range;

#[derive(Copy, Clone)]
enum GraphemeWidth {
    Half,
    Full
}

impl From<GraphemeWidth> for usize {
    fn from(width: GraphemeWidth) -> Self {
        match width {
            GraphemeWidth::Half => 1,
            GraphemeWidth::Full => 2
        }
    }
}

struct Fragment {
    grapheme: String,
    render_width: GraphemeWidth,
    replacement: Option<char>
}

pub struct Line {
    fragments: Vec<Fragment>
}

impl Line {
    pub fn len(&self) -> usize {
        self.fragments.len()
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let mut result = String::new();
        if range.start >= range.end {
            return result;
        }

        let mut current_pos: usize = 0;
        for f in self.fragments.iter() {
            if current_pos >= range.end { break; }
            let f_end = current_pos.saturating_add(f.render_width.into());
            if f_end > range.start {
                if f_end > range.end || current_pos < range.start {
                    // Cut off on either right or left respectively
                    result.push('⋯');
                } else if let Some(char) = f.replacement {
                    result.push(char);
                } else {
                    result.push_str(&f.grapheme);
                }
            }

            current_pos = f_end;
        }

        result
    }

    pub fn width(&self, range: Range<usize>) -> usize {
        self.fragments
            .iter()
            .skip(range.start)
            .take(range.end.saturating_sub(range.start))
            .map(|f| -> usize { f.render_width.into() })
            .sum()
    }
}

impl From<&str> for Line {
    fn from(txt: &str) -> Self {
        Self {
            fragments: txt.graphemes(true).map(|g| {
                let unicode_width = g.width();
                let render_width = match unicode_width {
                    0 | 1 => GraphemeWidth::Half,
                    _ => GraphemeWidth::Full
                };
                let replacement = match unicode_width {
                    0 => Some('·'),
                    _ => None
                };
                Fragment {
                    grapheme: g.to_string(),
                    render_width,
                    replacement
                }
            }).collect()
        }
    }
}