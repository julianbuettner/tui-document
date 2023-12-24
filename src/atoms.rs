use std::iter::repeat;

use ratatui::text::Line;

use crate::DocumentBlock;

pub struct Newlines {
    count: usize,
}

impl Default for Newlines {
    fn default() -> Self {
        Self { count: 1 }
    }
}

impl Newlines {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

impl DocumentBlock for Newlines {
    fn max_width(&self) -> usize {
        0
    }
    fn width_pressure(&self) -> usize {
        0
    }
    fn get_text(&self, _width: usize) -> ratatui::text::Text {
        ratatui::text::Text {
            lines: repeat(Line::default()).take(self.count).collect(),
        }
    }

    fn box_size_given_width(&self, _width: usize) -> (usize, usize) {
        (0, self.count)
    }
}
